# Suported Widgets

All the supported wigdets in yucky-ewwii. **Do note that the properties of each widget follows the ewwii property names.** If a property is not working, then refer to the properties of the actual widget provided here: [Ewwii Docs](https://ewwii-sh.github.io/docs/widgets/props).

## all

These properties apply to all widgets, and can be used anywhere!

**Properties**

- `class`: `string` css class name
- `valign`: `string` how to align this vertically. possible values: "fill", "baseline", "center", "start", "end"
- `halign`: `string` how to align this horizontally. possible values: "fill", "baseline", "center", "start", "end"
- `vexpand`: `bool` should this container expand vertically. Default: false
- `hexpand`: `bool` should this widget expand horizontally. Default: false
- `eval_ignore`: `bool` skip the automatic re-evaluation of this widget.
- `width`: `int` width of this element
- `height`: `int` height of this element
- `active`: `bool` If this widget can be interacted with
- `tooltip`: `string` tooltip text (on hover)
- `visible`: `bool` visibility of the widget
- `style`: `string` inline scss style applied to the widget
- `css`: `string` scss code applied to the widget
- `can_target`: `bool` make the widget targettable to pointer events.
- `focusable`: `bool` make widget focusable
- `widget_name`: `strong` custom widget name

## combo-box-text

**Properties**

- `items`: `vec` Items displayed in the combo box
- `timeout`: `duration` timeout of the command. Default: "200ms"
- `onchange`: `string` runs when an item is selected, replacing `{}` with the item

## expander

**Properties**

- `name`: `string` name of the expander
- `expanded`: `bool` sets whether it's expanded

## revealer

**Properties**

- `transition`: `string` animation name ("slideright", "slideleft", etc.)
- `reveal`: `bool` whether the child is revealed
- `duration`: `duration` how long the transition lasts. Default: "500ms"

## checkbox

**Properties**

- `checked`: `bool` initial checked state
- `timeout`: `duration` command timeout. Default: "200ms"
- `onchecked`: `string` command when checked
- `onunchecked`: `string` command when unchecked

## color-button

**Properties**

- `use_alpha`: `bool` use alpha channel
- `onchange`: `string` command on color select
- `timeout`: `duration` Default: "200ms"

## color-chooser

**Properties**

- `use_alpha`: `bool` use alpha channel
- `onchange`: `string` command on color select
- `timeout`: `duration` Default: "200ms"

## scale

**Properties**

- `flipped`: `bool` reverse direction
- `marks`: `string` draw marks
- `draw_value`: `bool` show value
- `value_pos`: `string` where to show value ("left", "right", etc.)
- `round_digits`: `int` number of decimal places
- `value`: `float` current value
- `min`: `float` minimum value
- `max`: `float` maximum value
- `timeout`: `duration` Default: "200ms"
- `onchange`: `string` command on change (use `{}` for value)
- `orientation`: `string` layout direction

## progress

**Properties**

- `text`: text to show over the progress
- `show_text`: whether to show the text
- `flipped`: `bool` reverse direction
- `value`: `float` progress (0–100)
- `orientation`: `string` layout direction

## circular-progress

**Properties**

- `value`: `float` the progression value, between (0-100)
- `start_at`: `float` the percentage that the circle should start at
- `thickness`: `float` the thickness of the circle
- `clockwise`: `bool` wether the progress bar spins clockwise or counter clockwise
- `fg_color`: `string` foreground color of circle
- `bg_color`: `string` background color of circle

## input

**Properties**

- `value`: `string` set current text
- `placeholder`: `string` set a placeholder text
- `onchange`: `string` command on change (use `{}` for value)
- `timeout`: `duration` Default: "200ms"
- `onaccept`: `string` command on Enter (use `{}` for value)
- `password`: `bool` obscure input

## button

**Properties**

- `label`: `string` label to show on the button
- `timeout`: `duration` Default: "200ms"
- `onclick`: `string` command on activation
- `onmiddleclick`: `string` command on middle click
- `onrightclick`: `string` command on right click

## image

**Properties**

- `path`: `string` image file path
- `content_fit`: `string` how the image should fit ("fill", "contain", "cover", "scaledown")
- `can_shrink`: `bool` whether the image can shrink or not
- `image_width`: `int` image width
- `image_height`: `int` image height
- `preserve_aspect_ratio`: `bool` keep aspect ratio
- `fill_svg`: `string` fill color for SVGs

## box

**Properties**

- `spacing`: `int` spacing between children
- `orientation`: `string` direction of children
- `space_evenly`: `bool` distribute children evenly

## overlay

**Properties**

_None_

## tooltip

**Properties**

_None listed_

## scroll

**Properties**

- `hscroll`: `bool` allow horizontal scrolling
- `vscroll`: `bool` allow vertical scrolling
- `propagate_natural_height`: `bool` use the natural height if true

## eventbox

**Properties**

- `orientation`: `string` layout direction
- `spacing`: `int` spacing between children
- `space_evenly`: `bool` distribute children evenly
- `timeout`: `duration` Default: "200ms"
- `onscroll`: `string` command on scroll (`{}` becomes direction)
- `onhover`: `string` command on hover
- `onhoverlost`: `string` command on hover exit
- `cursor`: `string` cursor type
- `ondropped`: `string` command on drop (`{}` is URI)
- `dragvalue`: `string` URI to drag from this widget
- `dragtype`: `string` type to drag ("file", "text")
- `onclick`: `string` command on click
- `onmiddleclick`: `string` command on middle click
- `onrightclick`: `string` command on right click
- `onkeypress`: `string` command on any key press (`{}` becomes the id of the key pressed)
- `onkeyrelease`: `string` command on any key release (`{}` becomes the id of the key released)

## label

**Properties**

- `text`: `string` text to display
- `truncate`: `bool` truncate text
- `limit_width`: `int` max characters to show
- `truncate_left`: `bool` truncate beginning
- `show_truncated`: `bool` show truncation
- `unindent`: `bool` strip leading spaces
- `markup`: `string` Pango markup
- `wrap`: `bool` wrap text
- `gravity`: `string` text gravity
- `xalign`: `float` horizontal alignment
- `yalign`: `float` vertical alignment
- `justify`: `string` text justification
- `wrap_mode`: `string` wrap mode ("word", "char", etc.)
- `lines`: `int` max lines (−1 = unlimited)

## calendar

**Properties**

- `day`: `float` selected day
- `month`: `float` selected month
- `year`: `float` selected year
- `show_heading`: `bool` show heading
- `show_day_names`: `bool` show day names
- `show_week_numbers`: `bool` show week numbers
- `onclick`: `string` command with `{0}`, `{1}`, `{2}` for day/month/year
- `timeout`: `duration` Default: "200ms"

## stack

**Properties**

- `selected`: `int` child index
- `transition`: `string` animation name
- `transition_duration`: `int` duration in millisecond as number

## graph

**Properties**

- `value`: `float` current value
- `type`: `string` graph type. possible values: "line" (default), "step-line", "fill", "step-fill"
- `time_range`: `duration` the range of time to show. Default: "10s"
- `min`: `float` minimum value
- `max`: `float` maximum value
- `dynamic`: `bool` whether the y range should dynamically change based on value. Default: true
- `animate`: `bool` enable smooth scrolling. Default: true
- `flip_x`: `bool` flip the graph horizontally. Default: false
- `flip_y`: `bool` flip the graph horizontally. Default: false
- `vertical`: `bool` if set to true, the x and y axes will be exchanged. Default: false
- `thickness`: `float` the thickness of the line (for line charts). Default: 1.0
- `line_style`: `string` changes the look of the edges in the graph (for line charts). possible values: "miter" (default), "bevel", "round"
