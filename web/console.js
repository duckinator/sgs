if (location.search == "?debug") {
    // Intercept the console, so I can debug iOS Safari without having a Mac.

    let style_tag = document.createElement("style");
    style_tag.innerText = "#console:target ~ canvas { display: none; }\n#console { padding: 1em; }";

    let console_div = document.createElement("div");
    console_div.id = 'console';

    let console_toggle = document.createElement("button");
    console_toggle.innerText = "Console";
    console_toggle.style = "position: absolute; left: 10px; bottom: 10px; width: 100px; height: 83px;";
    console_toggle.onclick = _ =>
        document.location.hash = (document.location.hash == '#console') ? '' : '#console';

    document.head.append(style_tag);
    document.body.prepend(console_div);
    document.body.appendChild(console_toggle);

    let console = (cons => {
        let cfn = (func, text) => {
            let pre = document.createElement("pre");
            pre.innerText = func + ": " + text;
            console_div.appendChild(pre);
            cons[func].apply(text);
        };

        return {
            log: text => cfn("log", text),
            debug: text => cfn("debug", text),
            info: text => cfn("info", text),
            warn: text => cfn("warn", text),
            error: text => cfn("error", text),
        };
    })(window.console);
    window.console = console;
}
