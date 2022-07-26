import "bootstrap/dist/css/bootstrap.css";

import("./pkg").then(module => {
    module.run_app();
});