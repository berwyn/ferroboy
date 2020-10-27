(function() {var implementors = {};
implementors["eyre"] = [{"text":"impl Error for InstallError","synthetic":false,"types":[]}];
implementors["ferroboy"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for CartridgeLoadError","synthetic":false,"types":[]},{"text":"impl Error for DisassemblyError","synthetic":false,"types":[]},{"text":"impl Error for OperationError","synthetic":false,"types":[]}];
implementors["pico_args"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["tracing_core"] = [{"text":"impl Error for SetGlobalDefaultError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelError","synthetic":false,"types":[]},{"text":"impl Error for ParseLevelFilterError","synthetic":false,"types":[]}];
implementors["tracing_error"] = [{"text":"impl&lt;E&gt; Error for TracedError&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Error + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tracing_subscriber"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for TryInitError","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()