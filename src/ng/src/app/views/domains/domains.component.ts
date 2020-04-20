import {Component, OnInit, ViewChild} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import { FormControl, FormGroup } from '@angular/forms';
import { getStyle, hexToRgba } from '@coreui/coreui-pro/dist/js/coreui-utilities';
import { CustomTooltips } from '@coreui/coreui-plugin-chartjs-custom-tooltips';

import {ToastrService} from 'ngx-toastr';
import {DomainService, DomainStatesList, TableData} from '../../services/';
import {AbstractComponent} from '../abstract/component.abstract';

@Component({
  templateUrl: 'domains.component.html'
})
export class DomainsComponent extends AbstractComponent {

  public data: TableData;
  public statesList: DomainStatesList;
  public filterQuery = '';

  @ViewChild('editModal')
  public editModal;

  public currentForm: FormGroup = new FormGroup({
    id: new FormControl(),
    name: new FormControl(),
    url: new FormControl(),
    state: new FormControl(),
    author: new FormControl()
  });

  constructor(public toastr: ToastrService,
              public router: Router,
              public activatedRoute: ActivatedRoute,
              private domainService: DomainService) {
    super(toastr, router, activatedRoute);

    this.domainService.getStatesList()
      .subscribe(
        (statesList: DomainStatesList) => this.statesList = statesList
      )

    this.domainService.getList()
      .subscribe(
        (data: TableData) => {
          setTimeout(() => {
            this.data = [...data]
            },200);
        }, // success path
        error =>  this.handleServerError(error) // error path
      );

  }

  addOpen(){
    this.editModal.show();
  }

  cancel(){
    this.editModal.hide()
  }

  submit(form){
    console.log(form);
  }

}
