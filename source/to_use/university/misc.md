# TODO

- Definizione di naturali, interi, razionali, reali
- LCM : controllare e usare ^\exceptzero sia in lcm che gcd
- Comando per il periodo di un gruppo ||
- \complexabs{}
- use \eulernumber throughout
- \complexsin
- sinh^-1 = log(x + sqrt(x^2 + 1)) e
  cosh^-1 = log(x \pm sqrt(x^2 - 1)) per x >= 1 (due soluzioni)
  se invece x < 1 non ci sono soluzioni
- f strettamente crescente (decrescente) => f^-1 strettamente crescente (decrescente)
- tutti i limiti notevoli e.g. (1-cos x)/cos x
      1 - cos x asympt 1/2 x^2
- 1_G identity element of s group/monoid and such
- definizione di differentiable
- Syntax nel gruppo \in è come dire nell'insieme. No per struttura algebrica in generale
- definition of sine,cosine,tangent + inverses + hyperbolic inverses.
- definition of simple group$
- turing complete
- \def\int{\mathop{\fam\z@ int}\limits}
- semidirect product \ltimes and replace \ltimes + \rtimes
- generic cyclc group C_n command
- per i gruppi non semplici è possibile fare il quoziente (non banale) e quindi possibile carpire più informazioni.
- la cosa F_n = F_m se e solo se n=m
- esiste iniettiva X->Y se e solo se esiste Y-> X suriettiva.
- group index & \elementperiodtext
- desmos 3d lib
- \boundary, \interior etc. senza il secondo argomento
- i \cartesianprod i direct/semidirect products, dovrebbero linkare al direct inner/outer product
- hopital H =
- mettere il comando per linear subspace
  possiamo anche non usare underset su lim e sup ma direttamente \sup_

# AI promps
- Translate this LaTeX code from italian to english.
Maintain the same content and commands used. Just give me the translated code.
Here is/are the snippet/snippets:

- Read this math proposition and generate a coincide ID describing it.
Also generate a title for it with the same information.
Your response must only contain the ID written in lower kebab case and the title.
The ID and the title must be in english.
For example: "internal-direct-external-product-isomorphism-theorem Isomorphism between internal and external direct product".
Here is the proposition:

- Ci troviamo nella repository con i miei appunti di matematica, organizzati
per il mio custom framework di appunti "stellar".
Abbiamo un universo "math", che contiene una lista di corsi.
Ogni corso contiene una lista di pagine (html), e ogni pagina contiene una lista di snippet.
Gli snippet vengono scritti in LaTeX e ognuno di essi ha un id univoco.
Tutte queste pagine si situano in @latex .
Oltre ai vari snippet di tipo definizione, teorema, proposizione, lemma, dimostrazione etc, a volte si usano dei "plain" snippet "enviroment snippet" per includere delle spiegazioni o illustrazioni o intuizioni varie. A volte si può usare il comando \plain{} per includere direttamente del testo nella pagina senza associarlo ad uno snippet, ma attenzione che non puoi usare comandi latex lì dentro, al massimo tag html. L'idea è che ogni snippet sia self-contained e riutilizzabile.
I JSON di tutti i corsi sono in questa cartella @courses .
Nel file @definitions.sty  ho una serie di comandi speciali che linkano i vari oggetti alle loro definizioni. I comandi sono sia testuali che math mode. Se non sai se un comando esiste non usarlo a caso.
Nei lavori che ti chiederò di fare potrai chiedermi di: creare nuovi comandi per definizioni che vengono usate spesso; creare nuove pagine se necessario; importare immagini da aggiungere. (O farlo tu se vuoi).
Quando vuoi fare una lista di punti, per esempio elencare delle proprietà,
usa \emph al posto che \textbf.
A volte ha senso modificare l'ordine delle cose per avere pagine che mantengano strettamente un tema, pur mantenendo la dipendenza funzionale di introdurre nella prima prima ciò che si usa, ovviamente.
Usa sempre il mio stesso stile di scrittura in LaTeX e scrivi sempre in inglese.
Non hai il permesso di fare un overhaul delle mie dimostrazioni etc., solo se strettamente necessario in caso di errori, sviste o mancanze.
Sii preciso con le definizioni e le dimostrazioni e quando uso un teormea /lemma in una dimostrazione, lo puoi linkare usando snippetref (e.g \snippetref[set-equivalence-with-subsets][set equivalence with subsets]), cioè il primo argomento è l'ID e il secondo è il testo a schermo.
Guarda anche i comandi in @bettelini.sty, per esempio per fare le dimostrazioni con \iffproof o simili, o integrali.
Mantieni sempre quello che scrivo io a meno che non sia strettamente necessario cambiare qualcosa per mancanze o errori, quindi mantieni anche la stessi sintassi e struttura e spaziatura
Ogni volta che c'è una contraddizione termina la frase con \lighning.
Ecco una serie di pagine che puoi leggere per imparare la sintassi:
@AbstractAlgebraCosets.tex  @TopologyExercises.tex  @AbstractAlgebraSubgroupTypes.tex  @ProductCoproduct.tex  @Graphs.tex  @LinearAlgebraSystemLinearEquations.tex @InfimumAndSupremum.tex @CalculusTaylorSeries.tex @ComplexAnalysisRoots.tex @ComplexAnalysisEulerIdentity.tex 
Ecco alcuni tipi di lavori che potrei chiederti:
sto trascrivemtno i miei appunti scritti in classe nel mio framework.
Le cose che trascrivi dai miei appunti al mio sito vanno wrappare in \begin{radioactive} ... \end{radioactive} nel file originare, per marcare che sono già state trascritte.
Non voglio una trascrizione "uno a uno", se ci sono errori di logica, di battituta, o piccole cose puoi sistemarli, o se c'è qualcosa di non prettamente inerente al topic puoi toglierlo, ma la sostanza deve esserci. Non puoi togliere pezzi sostanziosi delle cose che scrivo.  Non seguire nemmeno troppo i miei capitoli dei miei appunti. Se c'è bisogno / ci sono dei TODO o errori, sistema / completa le dimostrazioni etc.
Potrei chiederti di refactorare alcune cose che ho già scritto, cercare errori, lavorare
sulla diffusione dei simboli con link.
Per adesso non ti dò nessun lavoro, studiati bene l'architettura e la sintassi di stellar e poi quando sei pronto ti propongo le mansioni sulla codebase.