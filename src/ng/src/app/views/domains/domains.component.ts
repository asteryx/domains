import {Component, OnInit, ViewChild} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {FormControl, FormGroup, Validators} from '@angular/forms';
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

  private urlPattern: RegExp = /(https?:\/\/)([\da-z.-]+)\.([a-z.]{2,6})[/\w .-?=]*\/?/;

  @ViewChild('editModal')
  public editModal;

  public currentForm: FormGroup = new FormGroup({
    id: new FormControl(null),
    name: new FormControl("", [Validators.required]),
    url: new FormControl("", [Validators.required, Validators.pattern(this.urlPattern)]),
    state: new FormControl(1, [Validators.required])
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

    this.updateDomanList()

  }

  get frm() { return this.currentForm.controls; }

  updateDomanList(){
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

  modalOpen(){
    this.currentForm.reset();
    this.editModal.show();
  }

  modalCancel(){
    this.editModal.hide()
  }

  submit(){
    console.log(this.currentForm.value);
    let data = this.currentForm.value;
    if (data['id'] !== null){
      data['id'] = Number.parseInt(data['id']);
    }
    data['state'] = Number.parseInt(data['state']);

    this.domainService.create(data)
      .subscribe(
        (res) => {
          this.updateDomanList();
          this.modalCancel();
        },
        (err) => this.handleServerError(err)
      )
  }

}
