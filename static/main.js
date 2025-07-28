import init, { World, set_panic_hook } from "./cgol.js"

const CELL_SIZE = 8

await init().then(function () {
    set_panic_hook()

    /** @type {JQuery<HTMLCanvasElement>} */
    const canvas = $("#canvas")
    const ctx = canvas[0].getContext("2d")
    ctx.imageSmoothingEnabled = false

    /** @type {World} */
    let world = new World(ctx, 0, 0, CELL_SIZE, CELL_SIZE, true);

    const auto = $("#auto")

    $("#world_w").on("change", function () {
        const width = $(this).val()
        world.width = width
    }).trigger("change")

    $("#world_h").on("change", function () {
        const height = $(this).val()
        world.height = height
    }).trigger("change")

    $("#wrap_around").on("change", function () {
        world.wrap_around = $(this).prop("checked")
    }).trigger("change")

    $("#run_once").on("click", function () {
        world.update()
    })


    $("#canvas").on("click", function (e) {
        let mouse_pos = get_mouse_pos(e)
        world.toggle_cell(mouse_pos.x, mouse_pos.y)
    })


    function get_mouse_pos(evt) {
        let rect = canvas[0].getBoundingClientRect()
        let scaleX = canvas.attr("width") / rect.width
        let scaleY = canvas.attr("height") / rect.height
        return {
            x: (evt.clientX - rect.left) * scaleX / CELL_SIZE,
            y: (evt.clientY - rect.top) * scaleY / CELL_SIZE
        }
    }
})
