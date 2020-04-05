import { Injectable, ViewContainerRef } from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import { map } from 'rxjs/operators';

@Injectable()
export class StatisticService {

  constructor(public http: HttpClient) {
  }

  private _handle(url: string, data: any) {
    return this.http.get(url, data);
  }

  getStatistic(data: {
    dtStart?: Date,
    dtEnd?: Date,
    domainList?: number[]
  } = {}) {
    let params: HttpParams = new HttpParams();

    if (data.dtStart !== null && data.dtStart !== undefined){
      params = params.set('dt_start', data.dtStart.toJSON());
    }

    if (data.dtEnd !== null && data.dtEnd !== undefined){
      params = params.set('dt_end', data.dtEnd.toJSON());
    }

    if (data.domainList !== null && data.domainList !== undefined){
      params = params.set('domain_list', `[${data.domainList}]`);
    }

    return this.http.get('api_user/statistic/', {params});
  }
}
