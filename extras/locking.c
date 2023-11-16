
// Ingo Molnar's patch to the linux kernel's first versions of mutexes support
DEFINE_MUTEX(name);

mutex_init(mutex);

void mutex_lock(struct mutex *lock);
int mutex_lock_interruptible(struct mutex *lock);
int mutex_trylock(struct mutex *lock);
void mutex_unlock(struct mutex *lock);
int mutex_is_locked(struct mutex *lock);