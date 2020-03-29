import { Component } from '@angular/core';
import { User, LoginForm } from '../../app.models';
import {AuthenticationService} from '../../services/authentication.service';
import { ToastrService } from 'ngx-toastr';
import {ActivatedRoute, Params, Router} from '@angular/router';
import {AbstractComponent} from '../abstract/component.abstract';

@Component({
  selector: 'app-dashboard',
  templateUrl: 'login.component.html'
})
export class LoginComponent extends AbstractComponent{
  constructor(public toastr: ToastrService,
              public router: Router,
              public activatedRoute: ActivatedRoute,
              private authService: AuthenticationService) {
    super(toastr, router, activatedRoute);
  }
  loginForm = new LoginForm();
  isLoading = false;

  onSubmit() {
    this.isLoading = !this.isLoading;

    this.authService.login(this.loginForm).subscribe(
      res => {
        this.isLoading = !this.isLoading;
        this.user.login(res);
        console.log(res);
        this.activatedRoute.queryParams.subscribe((params: Params) => {
          const to = params.returnUrl || 'dashboard';
          this.router.navigate([to]);
        });
      },
      error => this.handleServerError(error)
    );
  }
}
