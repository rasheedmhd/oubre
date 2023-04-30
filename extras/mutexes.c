typedef int semaphore; 
semaphore resource 1; 

void process A(void) { 
    down(&resource 1); 
    use resource 1( ); 
    up(&resource 1); 
} 


typedef int semaphore;
semaphore resource 1;
semaphore resource 2;

void process A(void) {
    down(&resource 1);
    down(&resource 2);
    use both resources( );
    up(&resource 2);
    up(&resource 1);
}

// Ingo Molnar's patch to the linux kernel's first versions of mutexes support
DEFINE_MUTEX(name);

mutex_init(mutex);

void mutex_lock(struct mutex *lock);
int mutex_lock_interruptible(struct mutex *lock);
int mutex_trylock(struct mutex *lock);
void mutex_unlock(struct mutex *lock);
int mutex_is_locked(struct mutex *lock);