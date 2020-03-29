import { Component } from '@angular/core';
import { User, LoginForm } from '../../app.models';
import {AuthenticationService} from '../../services/authentication.service';
import { ToastrService } from 'ngx-toastr';
import {ActivatedRoute, Params, Router} from '@angular/router';
import {AbstractComponent} from '../abstract/component.abstract';

@Component({
  selector: 'logout',
  template: '<div></div>'
})
export class LogoutComponent extends AbstractComponent{
  constructor(public toastr: ToastrService,
              public router: Router,
              public activatedRoute: ActivatedRoute,
              private authService: AuthenticationService) {
    super(toastr, router, activatedRoute);
  }
  user: User;
  loginForm = new LoginForm();
  isLoading = false;

  ngOnInit() {
    this.user.logout();
    this.router.navigate(['/login']);
  }

}
