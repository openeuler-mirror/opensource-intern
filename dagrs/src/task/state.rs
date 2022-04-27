use anymap::{CloneAny, Map};

type DMap = Map<dyn CloneAny + Send + Sync>;

/// Describe task's running result
pub struct ExecState {
    success: bool,
    retval: Retval,
}

/// Task's return value
pub struct Retval(Option<DMap>);

/// Task's input value
pub struct Inputval(Vec<Option<DMap>>);

impl ExecState {
    /// Get a new [`ExecState`].
    /// 
    /// `success`: task finish without panic?
    /// 
    /// `retval`: task's return value
    pub fn new(success: bool, retval: Retval) -> Self {
        Self { success, retval }
    }

    /// Get empty [`ExecState`].
    pub fn empty() -> Self {
        Self {
            success: false,
            retval: Retval::empty(),
        }
    }

    /// Get [`ExecState`]'s return value.
    /// 
    /// This method will clone [`DMap`] stored in [`ExecState`]'s [`Retval`].
    pub fn get_dmap(&self) -> Option<DMap> {
        self.retval.0.clone()
    }

    /// The task execution succeed or not.
    /// 
    /// `true` means no panic occurs.
    pub fn success(&self) -> bool {
        self.success
    }
}

impl Retval {
    #[allow(unused)]
    /// Get a new [`Retval`].
    /// 
    /// Since the return value may be transfered between threads,
    /// [`Send`], [`Sync`], [`CloneAny`] is needed.
    pub fn new<H: Send + Sync + CloneAny>(val: H) -> Self {
        let mut map = DMap::new();
        assert!(map.insert(val).is_none(), "[Error] map insert fails.");
        Self(Some(map))
    }

    /// Get empty [`Retval`].
    pub fn empty() -> Self {
        Self(None)
    }
}

impl Inputval {
    /// Get a new [`Inputval`], values stored in vector are ordered 
    /// by that of the given [`TaskWrapper`]'s `rely_list`.
    pub fn new(vals: Vec<Option<DMap>>) -> Self {
        Self(vals)
    }

    #[allow(unused)]
    /// This method get needed input value from [`Inputval`].
    pub fn get<H: Send + Sync + CloneAny>(&self, index: usize) -> Option<&H> {
        if let Some(Some(dmap)) = self.0.get(index) {
            dmap.get::<H>()
        } else {
            None
        }
    }
}
