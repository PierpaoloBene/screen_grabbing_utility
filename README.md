# screen_grabbing_utility


Screen grabbing utility 

Cose da fare per il progetto:
- [x] Ritaglio immagine in post processing;
- [x] Gestione più monitor;
    - [x] Testare su Mac 
    - [x] Testare su più schermi
    - [x] Risolvere 'problema' risoluzione diversa dal 100%

- [x] Finestra Setting , con casella testo per scegliere path (e path di default) e radio button per scegliere formato , salvataggio;
- [x] Capire come salvare immagine con edit senza fare screen;

Aggiustamenti:
- [x] Rendering shapes in tempo reale e evitare sfasamenti;
- [x] Numeri fissi a cazzo per dimensioni finestra, prendendo dimensioni finestra da frame;
- [x] Cambiare puntatori del mouse nelle varie fasi (rettandgolo ,testo)
- [x] Sistemare scrittura testo (posizionamento, dimensione a scelta) 
- [x] Capire come funziona la selected window di buffer
- [x] Aggiungere testo : “premere ctrl+D per fare screen” 


Modulare:
- [x]Take_Screenshot: funzione a cui delegare la capture. (Width,height,current_os,rect_pos ,….)  
- [x]Save_Screenot:  solo per fare il save;
- [x]Define_Rectangle: definire rettangolo  della mouse_pos ( da mettere dopo mouse_pos = ui.input) , da diff_x e diff_y 
- [x]Init: Funzione Central panel select window 1 da delegare.
- [x]Refactor_on_windows: da chiamare negli if current_os==windows, modifica il self height e with; 
- [x]Load_image : Funzione per caricare immagine. 

