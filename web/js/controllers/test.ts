import { Controller } from "@hotwired/stimulus";

export class TestController extends Controller {
  initialize() {
    console.log("test");
  }
}
