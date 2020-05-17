import {Component, OnInit, ViewChild} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import { FormControl, FormGroup } from '@angular/forms';
import { getStyle, hexToRgba } from '@coreui/coreui-pro/dist/js/coreui-utilities';
import { CustomTooltips } from '@coreui/coreui-plugin-chartjs-custom-tooltips';
import {BaseChartDirective} from 'ng2-charts/ng2-charts';
import {ToastrService} from 'ngx-toastr';
import {DateRange} from '../../app.models';
import {StatisticService} from '../../services/';
import {AbstractComponent} from '../abstract/component.abstract';





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
  private generatedStart: Date;
  private generatedEnd: Date;

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
    this.updateStats();
  }

  dateStartChange(value: any){
    this.dateRange.dateStart = value;
    this.updateStats();
  }

  dateEndChange(value: any){
    this.dateRange.dateEnd = value;
    this.updateStats();
  }

  radioChange(value){
    // console.log(this.divisorsMatch[value]);
    this.updateStats();
  }

  updateStats(){
    this.statService.getStatistic(this.currentForm.value)
      .subscribe(
      res => this.refreshChart(res),
      error => this.handleServerError(error)
    )
  }

  formatDate(inputDate: Date){
    let date = new Date();

    if (inputDate !== undefined) {
      date = new Date(inputDate);
    }

    date.setSeconds(0);
    date.setMilliseconds(0);

    if (this.divider === "hour"){
      date.setMinutes(0);
    }

    if(this.divider === "day"){
      date.setMinutes(0);
      date.setHours(0);
    }
    return date
    // return `${date.getDate()}.${date.getMonth() + 1}.${date.getFullYear()} ${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`
  }

  generateLabelData(): Array<Date>{
    let result: Array<Date> = [];

    let dtStart = this.currentForm.value.dtStart;
    let dtEnd = this.currentForm.value.dtEnd;

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
    let statusIndex: number = 0;
    let currentDate: Date;

    for (let index in this.chartLabels){
      let currentValue: number = 0;
      let countValues: number = 0;

      const labelDate = this.chartLabels[index];

      let loadTime: number = 0;
      let status = domainStatuses[statusIndex] || {};

      currentDate = this.formatDate(status.date);

      if (labelDate.toString() === currentDate.toString()){
        loadTime = status.loading_time;
        statusIndex += 1;
      }else if(currentDate < labelDate){

        while (currentDate < labelDate && statusIndex<=domainStatuses.length){
          currentValue += status.loading_time;
          countValues += 1;
          statusIndex += 1;
          status = domainStatuses[statusIndex] || {};
          currentDate = this.formatDate(status.date);
        }
        loadTime = currentValue / countValues;

      }
      result.push(loadTime);
    }
    return result;
  }

  refreshChart(domainData: any){
    this.chartLabels = this.generateLabelData();

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
