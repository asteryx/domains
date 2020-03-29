import { Component } from '@angular/core';
import { User, LoginForm } from '../../app.models';
import {AuthenticationService} from '../../services/authentication.service';
import { ToastrService } from 'ngx-toastr';
import {ActivatedRoute, Params, Router} from '@angular/router';

@Component({
  selector: 'app-dashboard',
  templateUrl: 'login.component.html'
})
export class LoginComponent {
  constructor(private authService: AuthenticationService,
              private toastr: ToastrService,
              private router: Router,
              private activatedRoute: ActivatedRoute) {
  }
  user: User;
  loginForm = new LoginForm();
  isLoading = false;

  onSubmit() {
    this.isLoading = !this.isLoading;
    this.authService.login(this.loginForm).subscribe(
      res => {
        this.isLoading = !this.isLoading;
        this.user = new User().login(res);
        console.log(res);
        this.activatedRoute.queryParams.subscribe((params: Params) => {
          const to = params.returnUrl || 'dashboard';
          this.router.navigate([to]);
        });
      },
      error => {
        if (error.status === 400) {
          this.toastr.error(error.error.msg, 'Error');
        } else {
          console.log(error.message);
        }
        // this.loginForm.password = null;
        this.isLoading = !this.isLoading;
      }
    );
  }
}
