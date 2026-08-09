#include "wasm_widget.h"
#include <stdlib.h>
#include <string.h>

static const char TEXT[] = "Hello from C";

bool exports_ratatui_widget_widget_render(
    exports_ratatui_widget_widget_rect_t *area,
    wasm_widget_list_u8_t *maybe_state,
    exports_ratatui_widget_widget_render_result_t *ret,
    exports_ratatui_widget_widget_widget_error_t *err)
{
    (void) area;
    (void) maybe_state;
    (void) err;

    size_t len = strlen(TEXT);
    exports_ratatui_widget_widget_render_command_t *commands =
        malloc(len * sizeof(exports_ratatui_widget_widget_render_command_t));
    if (!commands && len > 0) {
        return false;
    }

    for (size_t i = 0; i < len; i++) {
        char sym[2] = { TEXT[i], '\0' };
        commands[i].tag = EXPORTS_RATATUI_WIDGET_WIDGET_RENDER_COMMAND_CELL;
        commands[i].val.cell.x = (uint16_t) i;
        commands[i].val.cell.y = 0;
        wasm_widget_string_dup(&commands[i].val.cell.symbol, sym);
        commands[i].val.cell.fg.is_some = true;
        commands[i].val.cell.fg.val.tag = EXPORTS_RATATUI_WIDGET_WIDGET_COLOR_GREEN;
        commands[i].val.cell.bg.is_some = true;
        commands[i].val.cell.bg.val.tag = EXPORTS_RATATUI_WIDGET_WIDGET_COLOR_BLACK;
    }

    ret->commands.ptr = commands;
    ret->commands.len = len;
    ret->state.is_some = false;
    return true;
}

bool exports_ratatui_widget_widget_handle_event(
    exports_ratatui_widget_widget_event_t *event,
    bool *ret,
    exports_ratatui_widget_widget_widget_error_t *err)
{
    (void) event;
    (void) err;
    *ret = false;
    return true;
}

void exports_ratatui_widget_widget_capabilities(wasm_widget_list_string_t *ret) {
    ret->ptr = NULL;
    ret->len = 0;
}
