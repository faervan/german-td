# Game
## Visual Effects
- [ ] Add simple enemy damage effect (red enemy material tint)
- [ ] Add enemy spawn effect
- [ ] Add enemy death effect
- [ ] Add tower spawn + destruction effect

## Sound Effects
- [ ] Add fire tower charge and hit sounds

## UI
- [ ] Add `AppState::Menu`
    - [ ] Add map selection/progression system
- [ ] Add game over UI
### HUD
- [ ] Add player health HUD
- [ ] Add enemy health HUD
- [ ] Add tower cooldown HUD
- [ ] Spice up the gold HUD
- [ ] (Maybe) rework the tower placement/upgrade HUD

### Controls
- [ ] Add rotation
- [ ] Add zoom
- [ ] Add touch drag

## Enemy Assets
- [ ] Add armored enemy

## Tower Assets
- [x] Add ~bomb~/fire tower for AoE attacks
    - [x] Switch archer towers back to single target
- [ ] Add a `projectile_offset` field that specifies an offset (relative to the tower transform) at which the projectiles will be spawned
- [ ] Figure out a good mechanism for shooting animations (not just an animation of the tower gltf, but to allow smth like the "creation" of the projectile itself)

## Projectile Assets
- [ ] Make the `Arrow` projectile actually visible (bigger)
- [ ] Add fire ball projectile for the fire tower

# Editor
- [ ] Add enemy edit tab
- [ ] Add map edit tab
- [ ] Fill missing fields with default when loading assets

# Bugs
- [ ] Fix editor freezing when assets are hot reloaded
