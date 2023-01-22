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