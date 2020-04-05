import { Injectable, ViewContainerRef } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { StatisticForm } from '../app.models';
import { map } from 'rxjs/operators';

@Injectable()
export class StatisticService {

  constructor(public http: HttpClient) {
  }

  private _handle(url: string, data: any) {
    return this.http.get(url, data);
  }

  getStatistic(statForm: StatisticForm) {
    const data = {
      dt_start: 'eeeeeeeeee',
      dt_end: 'qqqqqqqqqqq'
    };
    return this.http.get('api_user/statistic/', {params: data});
  }
}
