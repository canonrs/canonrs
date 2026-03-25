# AspectRatio

## When to use
Use to maintain a consistent width/height ratio for media, embeds, maps, or any content that must preserve proportions.

## Common ratios
- `16:9` — video (default)
- `4:3` — legacy media
- `1:1` — square (avatar, thumbnail)
- `21:9` — ultrawide

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| width | f32 | 16.0 | Ratio width unit |
| height | f32 | 9.0 | Ratio height unit |
| class | String | "" | CSS class |
| id | Option<String> | None | HTML id |

## Behavior
- Uses CSS padding-top trick via primitive
- Children positioned absolutely to fill container

## Edge cases
- Always set explicit width on parent container
- Do not use for text content — only for media/embeds
