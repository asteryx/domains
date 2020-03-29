import { Injectable } from '@angular/core';
import { Router, CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot } from '@angular/router';
import { Observable } from 'rxjs';
import { User } from './app.models';

@Injectable()
export class AuthenticatedGuard implements CanActivate {
  private user: User = new User();
  constructor(private router: Router) { }

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot
    ): Observable<boolean>|Promise<boolean>|boolean {
      if (this.user.isLoggedIn()) {
        return true;
      }
      this.user.logout();
      // not logged in so redirect to login page with the return url
      this.router.navigate(['/login'], { queryParams: { returnUrl: state.url }});
      return false;
  }

  resolve(): void {
    if (this.user.isLoggedIn()) {
      this.router.navigate(['/dashboard']);
    }
  }

}

