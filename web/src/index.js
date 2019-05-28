import * as wasm from "term-image-wasm"
import { getStream, nearest_base } from "./utils"
import { DisplayMode, BlockStyle } from "./types"

import { Terminal } from "xterm"
import * as fit from "xterm/lib/addons/fit/fit"
import * as aspectratio from "aspectratio"

import "xterm/dist/xterm.css"
import "xterm/dist/xterm.js"
import "./index.css"

Terminal.applyAddon(fit)
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
        this.blockStyle = BlockStyle.EXTENDED
        this.mode = DisplayMode.BLOCK
    }
}

let options = new Options()

const create_ratio = (cell_width, cell_height) => {
    let [maxWidth, maxHeight] = [
        term.cols * cell_width,
        term.rows * cell_height
    ]
    return aspectratio.resize(1280, 720, maxWidth, maxHeight)
}

const snapshotVidToCanvas = () => {
    const vid = document.getElementById("video")
    const canvas = document.getElementById("canvas")
    const ctx = canvas.getContext("2d")

    let cell_width, cell_height
    switch (options.mode) {
        case DisplayMode.BLOCK:
            [cell_width, cell_height] = [4, 8]
            break
        case DisplayMode.BRAILLE:
            [cell_width, cell_height] = [2, 4]
            break
    }

    let [width, height] = create_ratio(cell_width, cell_height)

    width = nearest_base(width, cell_width)
    height = nearest_base(height, cell_height)

    canvas.width = width
    canvas.height = height
    vid.width = width
    vid.height = height

    ctx.drawImage(vid, 0, 0, width, height)

    term.writeln("\r".repeat(25))

    switch (options.mode) {
        case DisplayMode.BLOCK:
            term.writeln(
                wasm.render_blocks(
                    width,
                    height,
                    options.ansi,
                    options.blend,
                    options.blockStyle
                )
            )
            break
        case DisplayMode.BRAILLE:
            term.writeln(wasm.render_braille(width, height, options.ansi))
            break
    }

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

    document.getElementById("extended").onclick = function() {
        options.blockStyle = BlockStyle.EXTENDED
    }
    document.getElementById("slabs").onclick = function() {
        options.blockStyle = BlockStyle.SLABS
    }
    document.getElementById("half").onclick = function() {
        options.blockStyle = BlockStyle.HALVES
    }

    document.getElementById("block-mode").onclick = function() {
        options.mode = DisplayMode.BLOCK
    }
    document.getElementById("braille-mode").onclick = function() {
        options.mode = DisplayMode.BRAILLE
    }
}

const main = async () => {
    setupHooks()
    const stream = await getStream()
    const vid = document.getElementById("video")
    vid.srcObject = stream
    snapshotVidToCanvas()
}

main()
