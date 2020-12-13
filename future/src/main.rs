use {
    std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};


pub struct TimerFuture {

    estado_comp: Arc<Mutex<EstadoCompartido>>, 
}


struct EstadoCompartido {
    
    completo: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>{

        let mut estado_compartido = self.estado_comp.lock().unwrap();

        if estado_compartido.completo {
            Poll::Ready(())
        } else {
            estado_compartido.waker = Some(cx.waker().clone());
            Poll::Pending
        }

    }
}
impl TimerFuture {

    pub fn new(duracion: Duration) -> Self {
        let estado_compartido = Arc::new(Mutex::new(EstadoCompartido {
            completo: false,
            waker: None
        }));

        let hilo_estado_compartido = estado_compartido.clone();

        thread::spawn(move || {
            
            thread::sleep(duracion);
            let mut estado_compartido = hilo_estado_compartido.lock().unwrap();
            estado_compartido.completo = true;
            if let Some(waker) = estado_compartido.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { estado_compartido }

    }
}

fn main() {
    println!("Hello, world!");
}
