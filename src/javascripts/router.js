export function init_router(app) {
    var router = new Navigo();
    router.on('register', function() {
        app.innerHTML = '<object type="text/html" data="templates/register.html" ></object>';
    })
}