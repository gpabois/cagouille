use crate::event::{traits::Event, EventSlot};

#[derive(Default)]
pub struct ElementEvents<'a> {
    onabort:    Option<EventSlot<'a, OnAbort>>,
    onauxclick: Option<EventSlot<'a, OnAuxClick>>,
    onblur:     Option<EventSlot<'a, OnBlur>>,
    oncancel:   Option<EventSlot<'a, OnCancel>>,
    oncanplay:  Option<EventSlot<'a, OnCanPlay>>,
    oncanplaythrough: Option<EventSlot<'a, OnCanPlayThrough>>
}

pub struct OnAbort;
impl Event for OnAbort {
    type Payload = web_sys::Event;
}

pub struct OnAuxClick;
impl Event for OnAuxClick {
    type Payload = web_sys::MouseEvent;
}
pub struct OnBlur;
impl Event for OnBlur {
    type Payload = web_sys::FocusEvent;
}

pub struct OnCancel;
impl Event for OnCancel {
    type Payload = web_sys::AnimationPlaybackEvent;
}

pub struct OnCanPlay;
impl Event for OnCanPlay {
    type Payload = web_sys::Event;
}
pub struct OnCanPlayThrough;
impl Event for OnCanPlayThrough {
    type Payload = web_sys::Event;
}
pub struct OnChange;
pub struct OnClick;
pub struct OnClose;
pub struct OnContextMenu;
pub struct OnCueChange;
pub struct OnDblClick;
pub struct OnDrag;
pub struct OnDragEnd;
pub struct OnDragEnter;
pub struct OnDragExit;
pub struct OnDragLeave;
pub struct OnDragOver;
pub struct OnDragStart;
pub struct OnDrop;
pub struct OnDurationChange;
pub struct OnEmptied;
pub struct OnEnded;
pub struct OnError;
pub struct OnFocus;
pub struct OnFocusIn;
pub struct OnFocusOut;
pub struct OnFormData;
pub struct OnInput;
pub struct OnInvalid;
pub struct OnKeyDown;
pub struct OnKeyPress;
pub struct OnKeyUp;
pub struct OnLoad;
pub struct OnLoadedData;
pub struct OnLoadedMetadata;
pub struct OnLoadStart;
pub struct OnMouseDown;
pub struct OnMouseEnter;
pub struct OnMouseLeave;
pub struct OnMouseMove;
pub struct OnMouseOut;
pub struct OnMouseOver;
pub struct OnMouseUp;
pub struct OnPause;
pub struct OnPlay;
pub struct OnPlaying;
pub struct OnProgress;
pub struct OnRateChange;
pub struct OnReset;
pub struct OnResize;
pub struct OnScroll;
pub struct OnSecurityPolicyViolation;
pub struct OnSeeked;
pub struct OnSeeking;
pub struct OnSelect;
pub struct OnSlotChange;
pub struct OnStalled;
pub struct OnSubmit;
pub struct OnSuspend;
pub struct OnTimeUpdate;
pub struct OnToggle;
pub struct OnVolumeChange;
pub struct OnWaiting;
pub struct OnWheel;
pub struct OnCopy;
pub struct OnCut;
pub struct OnPaste;
pub struct OnAnimationCancel;
pub struct OnAnimationEnd;
pub struct OnAnimationIteration;
pub struct OnAnimationStart;
pub struct OnGotPointerCapture;
pub struct OnLoadEnd;
pub struct OnLostPointerCapture;
pub struct OnPointerCancel;
pub struct OnPointerDown;
pub struct OnPointerEnter;
pub struct OnPointerLeave;
pub struct OnPointerLockChange;
pub struct OnPointerLockError;
pub struct OnPointerMove;
pub struct OnPointerOut;
pub struct OnPointerOver;
pub struct OnPointerUp;
pub struct OnSelectionChange;
pub struct OnSelectStart;
pub struct OnShow;
pub struct OnTouchCancel;
pub struct OnTouchEnd;
pub struct OnTouchMove;
pub struct OnTouchStart;
pub struct OnTransitionCancel;
pub struct OnTransitionEnd;
pub struct OnTransitionRun;
pub struct OnTransitionStart;