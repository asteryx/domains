import { Injectable, ViewContainerRef } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {User, LoginForm} from '../app.models';
import { map } from 'rxjs/operators';

@Injectable()
export class AuthenticationService {

  constructor(public http: HttpClient) {
  }

  private _handle(url: string, data: any) {
    return this.http.post(url, data);
          // .pipe(map(res => res.json()));
  }

  // private getRandSuffics(){
  //
  //   let text = '';
  //   let possible = 'abcdefghijklmnopqrstuvwxyz0123456789';
  //
  //   for (var i = 0; i < 6; i++)
  //     text += possible.charAt(Math.floor(Math.random() * possible.length));
  //
  //   return text;
  //
  // }
  // private get device_id(){
  //   return device_id + this.getRandSuffics();
  // }

  login(loginForm: LoginForm) {
    const data = {
      email: loginForm.email,
      password: loginForm.password
    };
    return this._handle('api_user/auth/login/', JSON.stringify(data));
  }

  logout() {
    return this._handle('/user/logout', JSON.stringify({}));
  }
}
