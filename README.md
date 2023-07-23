Example demonstrating an `I64Vec2`-based `WorldPosition` scale. This example has a `WORLD_SCALE` of `10`, which means for every 1 unit of `bevy` `translation`, we have 10 units of `WorldPosition`.

`cargo run` or `cargo watch -x run`

`WASD` to move the camera's `WorldPosition`.

The general idea is that you deal with a very large integer scale (`i64`), avoiding issues with floating point arithmetic error. When doing computation, you can either:

1. increase the precision of the scale (multiply by X) and do the calculations in the integer scale, or
2. you can convert to a float scale then do the calculations in the float scale based on some world position reference point, then convert back to the integer scale: ideally you pick a reference point that is close to the objects being considered, e.g. the camera and the positions in the viewport.

Not covered, but should be easily adaptable:

1. Zoom In/Out
2. Optimisations, e.g. only "simulate"/"render" things inside the `Camera`'s "horizon" e.g. 2x the viewport

It's not really _infinite_, but the scale goes from

```rs
pub const MIN: i64 = i64::MIN; // -9_223_372_036_854_775_808i64
```

to

```rs
pub const MAX: i64 = i64::MAX; // 9_223_372_036_854_775_807i64
```

which is more than twice as many numbers as there are grains of sand on earth.

Of course, we divide by the world scale of `10` to encode a minimum precision, so it's only a `10`th of that, so maybe not quite as many numbers as there are grains of sand on earth.
