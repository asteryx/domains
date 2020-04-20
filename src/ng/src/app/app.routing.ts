import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

// Import Containers
import { DefaultLayoutComponent } from './containers';

import {AuthenticatedGuard} from './app.guards';
import {DashboardComponent} from './views/dashboard/dashboard.component';
import {DomainsComponent} from './views/domains/domains.component';
// import { P404Component } from './views/error/404.component';
// import { P500Component } from './views/error/500.component';
import { LoginComponent } from './views/login/login.component';
import { LogoutComponent } from './views/logout/logout.component';
// import { RegisterComponent } from './views/register/register.component';

export const routes: Routes = [
  {
    path: '',
    redirectTo: 'dashboard',
    pathMatch: 'full',
  },
  // {
  //   path: '404',
  //   component: P404Component,
  //   data: {
  //     title: 'Page 404'
  //   }
  // },
  // {
  //   path: '500',
  //   component: P500Component,
  //   data: {
  //     title: 'Page 500'
  //   }
  // },
  {
    path: '',
    resolve: [AuthenticatedGuard],
    children: [
      {
        path: 'login',
        component: LoginComponent,
        data: {
          title: 'Login Page'
        }
      },
    ]
  },
  {
    path: '',
    component: DefaultLayoutComponent,
    canActivate: [AuthenticatedGuard],
    data: {
      title: 'Home'
    },
    children: [
      // {
      //   path: 'base',
      //   loadChildren: './views/base/base.module#BaseModule'
      // },
      // {
      //   path: 'buttons',
      //   loadChildren: './views/buttons/buttons.module#ButtonsModule'
      // },
      // {
      //   path: 'charts',
      //   loadChildren: './views/chartjs/chartjs.module#ChartJSModule'
      // },
      {
        path: 'dashboard',
        component: DashboardComponent,
        data: {
          title: 'Dashboard'
        }
      },
      {
        path: 'domains',
        component: DomainsComponent,
        data: {
          title: 'Domains'
        }
      },
      // {
      //   path: 'forms',
      //   loadChildren: './views/forms/forms.module#FormsModule'
      // },
      // {
      //   path: 'google-maps',
      //   loadChildren: './views/google-maps/google-maps.module#GoogleMapsModule'
      // },
      // {
      //   path: 'icons',
      //   loadChildren: './views/icons/icons.module#IconsModule'
      // },
      // {
      //   path: 'notifications',
      //   loadChildren: './views/notifications/notifications.module#NotificationsModule'
      // },
      // {
      //   path: 'plugins',
      //   loadChildren: './views/plugins/plugins.module#PluginsModule'
      // },
      // {
      //   path: 'tables',
      //   loadChildren: './views/tables/tables.module#TablesModule'
      // },
      // {
      //   path: 'theme',
      //   loadChildren: './views/theme/theme.module#ThemeModule'
      // },
      // {
      //   path: 'apps',
      //   loadChildren: './views/apps/apps.module#AppsModule'
      // },
      // {
      //   path: 'widgets',
      //   loadChildren: './views/widgets/widgets.module#WidgetsModule'
      // }
      {
        path: 'logout',
        component: LogoutComponent,
        data: {
          title: 'Logout Page'
        }
      },
    ]
  }
];

@NgModule({
  imports: [ RouterModule.forRoot(routes) ],
  exports: [ RouterModule ]
})
export class AppRoutingModule {}
