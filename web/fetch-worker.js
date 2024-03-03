self.addEventListener("fetch", event => {
    event.respondWith(
        fetch(event.request)
        .catch(error => {
            return caches.match(event.request) ;
        })
    );
});
