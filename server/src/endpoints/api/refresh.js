let version = "";

fetchAsync("../../hot").then((start_version) => {
    version = start_version;

    function refresh() {
        fetchAsync("../../hot").then((version_new) => {
            if (version != version_new) { 
                version = version_new;
                buildBundle();
            }
        });

        setTimeout(refresh, 1000);
    }

    // initial call
    setTimeout(refresh, 1000);
});
