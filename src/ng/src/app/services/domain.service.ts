import { Injectable } from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import { retry } from 'rxjs/operators';

export interface DomainData {
  id: number;
  name: string;
  state: number;
  url: string;
  author: number;
}

export interface TableData extends Array<DomainData> {}

export interface DomainStateData {
  id: number;
  name: string;
}
export interface DomainStatesList extends Array<DomainStateData> {}


@Injectable()
export class DomainService {
  dataUrl = 'api_user/domain/';

  constructor(public http: HttpClient) {
  }

  private _handle(url: string, data: any) {
    return this.http.get(url, data);
  }

  getList(q: string = null) {
    let params: HttpParams = new HttpParams();

    if (q !== null){
      params = params.set('q', q.toString());
    }
    return this.http.get<TableData>(this.dataUrl, {params})
      .pipe(
        retry(3), // retry a failed request up to 3 times
      );
  }

  getStatesList(){
    return this.http.get<DomainStatesList>(`${this.dataUrl}states/`)
      .pipe(
        retry(3), // retry a failed request up to 3 times
      );
  }
}
