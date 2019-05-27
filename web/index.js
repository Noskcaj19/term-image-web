import * as wasm from "term-image-wasm"
import { getStream, nearest_base } from "./utils"

import { Terminal } from "xterm"
import * as fullscreen from "xterm/lib/addons/fullscreen/fullscreen"
import * as fit from "xterm/lib/addons/fit/fit"
import * as aspectratio from "aspectratio"

Terminal.applyAddon(fit)
Terminal.applyAddon(fullscreen)
let term = new Terminal()
window.term = term
term.open(document.getElementById("terminal"))
term.fit()
term.setOption("scrollback", 0)
term.setOption("disableStdin", true)

class Options {
    constructor() {
        this.fps = 15
        this.blend = true
        this.ansi = false
        this.extended = true
    }
}

var options = new Options()

const snapshotVidToCanvas = () => {
    const vid = document.getElementById("video")
    const canvas = document.getElementById("canvas")
    const ctx = canvas.getContext("2d")

    var [maxWidth, maxHeight] = [term.cols * 4, term.rows * 8]
    var [width, height] = aspectratio.resize(1280, 720, maxWidth, maxHeight)

    width = nearest_base(width, 4)
    height = nearest_base(height, 8)

    canvas.width = width
    canvas.height = height
    vid.width = width
    vid.height = height

    ctx.drawImage(vid, 0, 0, width, height)

    term.writeln("\r".repeat(25))
    term.writeln(
        wasm.render_blocks(
            width,
            height,
            options.ansi,
            options.blend,
            options.extended
        )
    )

    setTimeout(function() {
        window.requestAnimationFrame(snapshotVidToCanvas)
    }, 1000 / options.fps)
}

const setupHooks = () => {
    document.getElementById("fps").value = options.fps
    document.getElementById("fps").onkeyup = function() {
        let newFps = parseInt(this.value)
        if (!isNaN(newFps)) {
            options.fps = newFps
        }
    }
    document.getElementById("ansi").checked = options.ansi
    document.getElementById("ansi").onclick = function() {
        options.ansi = this.checked === true
    }
    document.getElementById("blend").checked = options.blend
    document.getElementById("blend").onclick = function() {
        options.blend = this.checked === true
    }
    document.getElementById("extended").checked = options.extended
    document.getElementById("extended").onclick = function() {
        options.extended = this.checked === true
    }
    // window.onresize = () => {
    //     term.fit()
    // }
}

const main = async () => {
    setupHooks()
    const stream = await getStream()
    const vid = document.getElementById("video")
    vid.srcObject = stream
    snapshotVidToCanvas()
}

main()
