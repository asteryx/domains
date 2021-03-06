import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HTTP_INTERCEPTORS, HttpClient, HttpHandler, HttpClientModule } from '@angular/common/http';
import { NgModule } from '@angular/core';
import { LocationStrategy, HashLocationStrategy } from '@angular/common';
import {
  AppAsideModule,
  AppBreadcrumbModule,
  AppHeaderModule,
  AppFooterModule,
  AppSidebarModule,
} from '@coreui/angular';
import { PerfectScrollbarModule } from 'ngx-perfect-scrollbar';
import { PERFECT_SCROLLBAR_CONFIG } from 'ngx-perfect-scrollbar';
import { PerfectScrollbarConfigInterface } from 'ngx-perfect-scrollbar';
const DEFAULT_PERFECT_SCROLLBAR_CONFIG: PerfectScrollbarConfigInterface = {
  suppressScrollX: true
};
import { DataTableModule } from 'angular2-datatable';
import { ModalModule } from 'ngx-bootstrap/modal';


import { AppComponent } from './app.component';
// Import containers
import { DefaultLayoutComponent } from './containers';
import {DashboardComponent} from './views/dashboard/dashboard.component';
import {DataFilterPipe} from './views/domains/datafilterpipe';
import {DomainsComponent} from './views/domains/domains.component';
// import { P404Component } from './views/error/404.component';
// import { P500Component } from './views/error/500.component';
import { LoginComponent } from './views/login/login.component';
// import { RegisterComponent } from './views/register/register.component';

const APP_CONTAINERS = [
  DefaultLayoutComponent
];

// Import routing module
import { AppRoutingModule } from './app.routing';

// Import 3rd party components
import { BsDropdownModule } from 'ngx-bootstrap/dropdown';
import { TabsModule } from 'ngx-bootstrap/tabs';
import { ChartsModule } from 'ng2-charts/ng2-charts';
import { LaddaModule } from 'angular2-ladda';
import { ToastrModule } from 'ngx-toastr';
import { BsDatepickerModule } from 'ngx-bootstrap/datepicker';
import { ButtonsModule } from 'ngx-bootstrap/buttons';

import {AuthenticationService, StatisticService, DomainService} from './services/';

import {Token, User} from './app.models';
import { HeadersInterceptor, ResponseInterceptor } from './app.interceptors';
import {AuthenticatedGuard} from './app.guards';
import {LogoutComponent} from './views/logout/logout.component';

@NgModule({
  imports: [
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule,
    BrowserModule,
    BrowserAnimationsModule,
    AppRoutingModule,
    AppAsideModule,
    AppBreadcrumbModule.forRoot(),
    AppFooterModule,
    AppHeaderModule,
    AppSidebarModule,
    ButtonsModule,
    PerfectScrollbarModule,
    BsDropdownModule.forRoot(),
    BsDatepickerModule.forRoot(),
    TabsModule.forRoot(),
    ChartsModule,
    LaddaModule.forRoot({
      // style: 'contract',
      spinnerSize: 40,
      spinnerColor: 'red',
      spinnerLines: 12
    }),
    ToastrModule.forRoot({
      timeOut: 3000,
      positionClass: 'toast-top-right',
      preventDuplicates: true,
    }),
    DataTableModule,
    ModalModule.forRoot()
  ],
  declarations: [
    AppComponent,
    ...APP_CONTAINERS,
    // P404Component,
    // P500Component,
    LoginComponent,
    LogoutComponent,
    // RegisterComponent
    DashboardComponent,
    DomainsComponent,
    DataFilterPipe,
  ],
  providers: [
    {
    provide: LocationStrategy,
    useClass: HashLocationStrategy
    },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: HeadersInterceptor,
      multi: true
    },
    {
      provide: HTTP_INTERCEPTORS,
      useClass: ResponseInterceptor,
      multi: true
    },
    HttpClient,
    AuthenticationService,
    StatisticService,
    DomainService,
    AuthenticatedGuard,
    User,
    Token,
  ],
  bootstrap: [ AppComponent ]
})
export class AppModule { }
