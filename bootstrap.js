import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-social/bootstrap-social.css"
import 'font-awesome/css/font-awesome.css';

import("./pkg").then(module => {
    module.run_app();
});