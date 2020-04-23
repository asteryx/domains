import {Component, OnInit, ViewChild} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import { FormControl, FormGroup } from '@angular/forms';
import { getStyle, hexToRgba } from '@coreui/coreui-pro/dist/js/coreui-utilities';
import { CustomTooltips } from '@coreui/coreui-plugin-chartjs-custom-tooltips';
import {BaseChartDirective} from 'ng2-charts/ng2-charts';
import {ToastrService} from 'ngx-toastr';
import {StatisticService} from '../../services/';
import {AbstractComponent} from '../abstract/component.abstract';

class DateRange{
  private _todayStart: Date;
  private _todayEnd: Date;

  private _startDateName: string = "startDate";
  private _endDateName: string = "endDate";

  constructor(){
    this._todayStart = new Date();
    this._todayStart.setHours(0, 0, 0);

    this._todayEnd = new Date();
    this._todayEnd.setHours(23, 59, 59);
  }

  private getKey(name: string): string{
    return `dashboard_${name}`
  }

  private getDateAttr(name: string, deflt: Date): Date{
    const keyName = this.getKey(name);
    let value: Date;

    let stringValue = localStorage.getItem(keyName);

    if(stringValue === null){
      value = deflt;
      localStorage.setItem(keyName, JSON.stringify(value));
    }else {
      value = new Date(JSON.parse(stringValue));
    }

    return value
  }

  private setDateAttr(name: string, value: Date){
    const keyName = this.getKey(name);
    localStorage.setItem(keyName, JSON.stringify(value));
  }

  public get dateStart(){
    return this.getDateAttr(this._startDateName, this._todayStart)
  }

  public get dateEnd(){
    return this.getDateAttr(this._endDateName, this._todayEnd)
  }

  public set dateStart(value: Date){
    this.setDateAttr(this._startDateName, value)
  }

  public set dateEnd(value: Date){
    this.setDateAttr(this._endDateName, value)
  }

}

@Component({
  templateUrl: 'dashboard.component.html'
})
export class DashboardComponent extends AbstractComponent implements OnInit {

  constructor(public toastr: ToastrService,
              public router: Router,
              public activatedRoute: ActivatedRoute,
              private statService: StatisticService) {
    super(toastr, router, activatedRoute);



    this.currentForm = new FormGroup({
      dtStart: new FormControl(this.dateRange.dateStart),
      dtEnd: new FormControl(this.dateRange.dateEnd)
    });
  }

  @ViewChild(BaseChartDirective)
  chart: BaseChartDirective;

  dateRange: DateRange = new DateRange();
  // todayStart: Date;
  // todayEnd: Date;

  currentForm: FormGroup;
  divisorsMatch: any = {
    min: 1,
    hour: 60,
    day: 60 * 24
  };
  divider:string = 'min';

  maxValue: number = 1000;

  public mainChartData: Array<any> = [
    {
      name: "nothing",
      data: [0]
    }
  ];
  /* tslint:disable:max-line-length */
  public chartLabels: Array<any>;
  /* tslint:enable:max-line-length */
  public mainChartOptions: any = {
    tooltips: {
      enabled: false,
      custom: CustomTooltips,
      intersect: true,
      mode: 'index',
      position: 'nearest',
      callbacks: {
        labelColor: (tooltipItem, chart) => {
          return { backgroundColor: chart.data.datasets[tooltipItem.datasetIndex].borderColor };
        }
      }
    },
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      xAxes: [{
        gridLines: {
          drawOnChartArea: false,
        },
        display: false,
      }],
      yAxes: [{
        ticks: {
          beginAtZero: true,
          maxTicksLimit: 5,
          stepSize: Math.ceil(this.maxValue / 5),
          max: this.maxValue
        }
      }]
    },
    elements: {
      line: {
        borderWidth: 2
      },
      point: {
        radius: 0,
        hitRadius: 10,
        hoverRadius: 4,
        hoverBorderWidth: 3,
      }
    },
    legend: {
      display: false
    }
  };

  public mainChartColours: Array<any> = [
    { // brandInfo
      backgroundColor: hexToRgba(getStyle('--info'), 10),
      borderColor: getStyle('--info'),
      pointHoverBackgroundColor: '#fff'
    }
  ];

  public random(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1) + min);
  }

  ngOnInit(): void {
    this.chartLabels = this.generateLabelData();
    this.updateStats();
  }

  dateStartChange(value: any){
    this.dateRange.dateStart = value;
    // this.updateStats();
  }

  dateEndChange(value: any){
    this.dateRange.dateEnd = value;
    // this.updateStats();
  }

  radioChange(value){
    console.log(this.divisorsMatch[value]);
  }

  updateStats(){
    this.statService.getStatistic(this.currentForm.value)
      .subscribe(
      res => this.refreshChart(res),
      error => this.handleServerError(error)
    )
  }

  formatDate(date: Date){
    return `${date.getFullYear()}_${date.getMonth()}_${date.getDay()}_${date.getHours()}_${date.getMinutes()}`
  }

  generateLabelData(): Array<String>{
    let result: Array<String> = [];

    let dtStart = this.currentForm.value.dtStart || this.dateRange.dateStart;
    let dtEnd = this.currentForm.value.dtEnd || this.dateRange.dateEnd;

    const step: number = this.divisorsMatch[this.divider] * (1000 * 60);

    // this.mainChartElements = Math.round((dtEnd - dtStart) / step );

    while (dtStart <= dtEnd){
      result.push(this.formatDate(dtStart));

      dtStart = new Date(dtStart.getTime()+(step));
    }


    return result
  }

  generateChartDomainData(domainStatuses: Array<any>): Array<any> {
     let result: Array<number> = [];

      for (let index in domainStatuses){
        const loadTime = domainStatuses[index].loading_time;

        this.maxValue = Math.max(this.maxValue, loadTime);
        result.push(loadTime);
      }
      return result;
  }

  refreshChart(domainData: any){
    this.mainChartData = [];
    for (let i = 0; i<=domainData.length - 1; i++){
      let domain = domainData[i];

      if (domain.statuses.length){
        this.mainChartData.push({
          data: this.generateChartDomainData(domain.statuses),
          label: domain.name
        })
      }
    }
    this.chart.chart.config.data.labels = this.chartLabels;
    this.chart.chart.config.data.datasets = this.mainChartData;
    this.chart.chart.update();  //datasets = this.mainChartData;
    // console.log(this.chart);
  }
}
