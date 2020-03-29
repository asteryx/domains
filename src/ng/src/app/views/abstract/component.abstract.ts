import {User} from '../../app.models';
import { NgForm } from '@angular/forms';
import { ToastrService } from 'ngx-toastr';
import {ActivatedRoute, Params, Router} from '@angular/router';

export class AbstractComponent {
  isLoading: boolean = false;
  user: User = new User();

  constructor(public toastr: ToastrService,
              public router: Router,
              public activatedRoute: ActivatedRoute) {
  }

  // @HostListener('window:scroll', ['$event'])
  // onScroll(event:any){
  //   centerSpinner();
  // }
  // @HostListener('window:resize', ['$event'])
  // onResize(event:any){
  //   centerSpinner();
  // }

  public handleServerError(resp: any) {
    this.isLoading = false;

    let errorCode = resp['code'] || 0;

    if (resp.status == 401 || errorCode){
      return
    }
    if (resp.status == 0){
      this.toastr.error('Error connecting to server');

      return
    }

    if (resp.error.msg) {
      this.toastr.error(resp.error.msg, 'Error');
      return;
    }

    this.toastr.error('Error connecting to server');
  }

}
