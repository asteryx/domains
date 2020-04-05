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

