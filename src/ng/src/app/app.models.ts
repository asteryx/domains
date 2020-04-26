import {Injectable} from '@angular/core';

@Injectable()
export class Token {
  private tokenName = 'Token';

  get token() {
    return localStorage.getItem(this.tokenName) || '';
  }

  set token(token: string) {
    if (token.length > 0) {
      localStorage.setItem(this.tokenName, token);
    }
  }

  remove() {
    localStorage.removeItem(this.tokenName);
  }
}

@Injectable()
export class User {
  public token: Token = new Token();
  get name() {
    return localStorage.getItem('username') || '';
  }
  set name(name: string) {
    if (name.length > 0) {
      localStorage.setItem('username', name);
    }
  }

  get email() {
    return localStorage.getItem('email') || '';
  }
  set email(name: string) {
    if (name.length > 0) {
      localStorage.setItem('email', name);
    }
  }

  isLoggedIn() {
    return !!this.token.token;
  }

  login(opt: any = {}) {
    if (opt !== null) {
      this.email = opt.email || '';
      this.name = opt.name || '';
      this.token = new Token();
      this.token.token = opt.token || '';
    }
    return this;
  }

  logout() {
    this.token.remove();
    localStorage.removeItem('username');
    localStorage.removeItem('email');
  }
}

export class LoginForm {
  constructor(
    public email: string = '',
    public password: string = ''
  ) {
  }
}

export class DateRange{
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
