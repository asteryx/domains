import { Injectable } from '@angular/core';
import { Token, User } from './app.models';
import {
  HttpRequest,
  HttpResponse,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
  HttpErrorResponse
} from '@angular/common/http';
import { Router } from '@angular/router';
import { Observable, timer } from 'rxjs';
import { tap } from 'rxjs/operators';


@Injectable()
export class HeadersInterceptor implements HttpInterceptor {
  constructor() {}
  private user = new User();

  intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {

    request = request.clone({
      setHeaders: this._get_headers(this.user.token)
    });
    return next.handle(request);
  }
  _get_headers(token: Token) {
    const headers = {'Content-Type': 'application/json'};
    const authKey = 'Authorization';

    if (token.token !== '') {
      headers[authKey] = `JWT ${token.token}`;
    }
    return headers;
  }
}


@Injectable()
export class ResponseInterceptor implements HttpInterceptor {
  constructor(public router: Router) {}
  intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {

    return next.handle(request).pipe(tap((event: HttpEvent<any>) => {
      if (event instanceof HttpResponse) {
        // do stuff with response if you want
      }
    }, (err: any) => {
      if (err instanceof HttpErrorResponse) {
        if (err.status === 401) {
          timer(100).subscribe(() => {
            if (this.router.url.slice(0, 6) !== '/login') {
                this.router.navigate(['/logout']);
              }
          });
        }
      }
    }));
  }
}
