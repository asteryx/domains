import { Injectable, ViewContainerRef } from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import { map } from 'rxjs/operators';
import {DomainStateData} from "./domain.service";

export interface StatusData {
  date: Date,
  loading_time: number,
  status_code: number,
  headers: String,
  filename: String,
}

export interface StatisticData {
  id: number;
  name: string;
  statuses: StatusData;
}

export interface StatisticList extends Array<StatisticData> {}


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

    return this.http.get<StatisticList>('api_user/statistic/', {params});
  }
}
