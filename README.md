# Scrype

Script your typing.

## What is this?

Scrype is a cross-platform program that watches what you type and runs scripts when it detects certain strings have been typed.

Those scripts are run in [Deno](https://github.com/denoland/deno), and can then perform actions and return text to type out in response.

For example you could type `shrug`, Scrype would erase that, and the script would type out `¯\_(ツ)_/¯`.

### Macros

Macros are defined as folders in the Scrype config directory.

Macros have configs that match text, and JavaScript scripts that run when those matches are met in the text you type.

### Prefixes & Suffixes

You can provide global prefixes and suffixes to matches. If you defined `;;` as your suffix, and had a script matching `shrug`, you would need to type out `shrug;;` to trigger it.

This feature exists because in alternatives such as [Espanso](https://github.com/espanso/espanso)--the inspiration for this project--, macros have to define prefixes and suffixes themselves, and often use different ones. Different people also prefer different setups. That problem frequently causes confusion when trying to memorize how to use macros you download.

## Features

✅ Implemented | ⏲️ Planned | ❔ Considering

| Feature | Status | How To Implement |
| :-: | :-: | :-: |
| [Deno](https://github.com/denoland/deno) Code Runner | ✅ |  |
| Native Code Runner | ❔ | Run compiled libraries in FFI. |
| Global Prefix & Suffix | ✅ |  |
| Macro Permissions With Deno | ⏲️ | Set Deno args in main config. |
| Text Pasting | ✅ |  |
| Direct Text Injection | ⏲️ | Write native bindings for each platform. |
| Text Matching | ✅ |  |
| RegEx Matching | ⏲️ | Match using RegEx in Rust, then pass the matched groups to the script. |
| [Pomsky](https://github.com/pomsky-lang/pomsky) Matching | ⏲️ | The same as RegEx, but use the Pomsky core to match instead. |
| Custom Deno Path | ⏲️ | Set path in main config. |
| Backspace To Undo | ⏲️ | Track how many characters are sent and the history match of the last macro. |
| Android | ❔ | Use accessibility API. |

## Ideas

### Setting Storage

You can use [`localStorage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage#examples) to save settings for specific scripts that persist between runs.

### GUI

You can use [Gluon](https://github.com/gluon-framework/gluon) or [WebView](https://github.com/webview/webview_deno) to show temporary GUIs to take more complex input.

Note that while these windows are open, all macros are paused and no more typing gets tracked.

#### Eval Example

Here's an example script that opens a window using WebView, takes input from it when the user presses enter, evaluates it using JavaScript, and pastes the result.

`index.ts`
```ts
import { SizeHint, Webview } from "https://deno.land/x/webview@0.7.5/mod.ts";

const html = new TextDecoder("utf-8").decode(
	Deno.readFileSync("./index.html")
);

const webview = new Webview(true, {
	width: 400,
	height: 50,
	hint: SizeHint.FIXED,
});
webview.title = "Eval";

webview.navigate(`data:text/html,${encodeURIComponent(html)}`);
webview.bind("resolve", (text) => {
	webview.destroy();
	console.log(`scrype_paste_text:${text}`);
});
webview.bind("exit", () => {
	webview.destroy();
});
webview.run();
```
`index.html`
```html
<html>
	<body style="margin: 0">
		<input
			style="
				width: 100vw;
				height: 100vh;
				padding: 0;
				margin: 0;
				outline: none;
				border: none;
				resize: none;
			"
			type="text"
			onkeydown="keys(event)"
			autofocus
		/>
		<script>
			function keys(event) {
				switch (event.key) {
					case "Enter":
						const evalFunction = new Function(`return (${event.target.value})`);
						let result = evalFunction();
						try {
							result = JSON.stringify(result);
						} catch {
							result = result.toString();
						}
						resolve(result);
						break;
					case "Escape":
						exit();
						break;
				}
			}
		</script>
	</body>
</html>
```

This is a very basic UI. You can make it as complex as you want.
