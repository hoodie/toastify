# macOS support

On macOS toastify uses `notify-rust`'s default backend, which wraps the
deprecated (but still working) `NSUserNotificationCenter` through
[`mac-notification-sys`](https://docs.rs/mac-notification-sys/).

## Supported options

| Option          | Maps to                                                   |
| --------------- | --------------------------------------------------------- |
| `title`, `body` | `Notification::summary()`, `Notification::body()`         |
| `--sound-name`  | `Notification::sound_name()`                              |
| `--subtitle`    | `Notification::subtitle()`                                |
| `--image`       | `Notification::image_path()` (content image on the right) |
| `--app-name`    | `set_application(get_bundle_identifier_or_default(..))`   |
| `--bundle-id`   | `set_application(..)`                                     |

`--app-name` is resolved to a bundle identifier; unknown names fall back to
`com.apple.Finder`. `--app-name` and `--bundle-id` are mutually exclusive.

## Left out for now

### Reachable through notify-rust, not yet exposed

- **Delayed delivery** (`--delay <SECONDS>`): `Notification::schedule_raw(timestamp)`
  sets `delivery_date`. Delivered asynchronously, nothing blocks.
- **Action buttons** (`--action ID:LABEL`): `Notification::action()` maps to a
  single main button or a dropdown (`MainButton::DropdownActions`) for several actions.
  Only useful in combination with waiting for the response.
- **Waiting for a response** (`--wait`): `NotificationHandle::wait_for_response()`
  blocks until the user interacts and yields `Default`, `Action(id)` or `Closed`,
  which could be printed to stdout for use in scripts.
  `NotificationHandle::on_close()` does the same for close events and adds a close button.

### Only available via mac-notification-sys directly

notify-rust doesn't expose these. We'd have to depend on `mac-notification-sys`
directly and build the notification ourselves on macOS:

- `app_icon`: custom icon on the left side
- `close_button`: close button with a custom label
- `MainButton::Response`: inline text reply field (`NotificationResponse::Reply`)
- `wait_for_click`: block until the notification body is clicked

### Requires the `preview-macos-un` feature

notify-rust's opt-in `UNUserNotificationCenter` backend supports urgency
(mapped to interruption levels), timeouts, ids and updating or closing delivered
notifications. It needs a signed app bundle with a bundle identifier and
notification permission, so it's not practical for a plain CLI binary yet.

### Not supported on macOS

`--urgency`, `--expire-time`, `--icon`, `--hint`, `--categories` and `--id` are
Linux/BSD (XDG) only and have no effect on the `NSUserNotificationCenter` backend.
