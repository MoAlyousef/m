// We can use string literals which contain html elements.
// We also use some bulma classes to style certain elements.
// To use them correctly, we use the element's inner_html() method, instead of text().
pub const ABOUT_ME: &str = r#"<h3>Mohammed Alyousef</h3>
    <p>
        I'm a neurosurgeon, specialized in skull base surgery, at King Abdulaziz University Hospital, who's interested in programming,
        linguistics and history.
        I mainly develop using C, C++ and Rust. I contribute to open source software.
        I'm also interested in linguistics, especially in comparative Semitic linguistics. I'm a
        language aficionado as well.
        I speak Arabic, English and French. Fields of history that interest me include ancient history
        of the Middle East.
        I also enjoy traveling and photography. I've lived in several countries:
    <ul>
        <li>
            Jeddah, Saudi Arabia
        </li>
        <li>
            Manchester, UK - 1 year
        </li>
        <li>
            Chicago, US - 1 year
        </li>
        <li>
            Paris, France - 8 years
        </li>
        <li>
            Brussels, Belgium - 1 year
        </li>
    </ul>
    </p>
    </div>
    <div class="columns is-mobile is-centered">
    <div class="column is-half">
        <img src="assets/image.jpg" alt="Mohammed Alyousef" width="250" height="250">
    </div>
    </div>
    <div>
        <h3>Details</h3>
        <p>
            <strong>Name:</strong><br>
            Mohammed Alyousef<br>
            <strong>Work:</strong><br>
            King Abdulaziz University
        </p>
        <strong>Location:</strong><br>
        Jeddah, Saudi Arabia </p>
    </div>"#;

pub const MY_RESUME: &str = r#"<h2>Resumé</h2>
    
    <hr>
    
    
    <div>
        <h5>
            Assistant professor and consultant neurosurgeon
        </h5>
        <p>
            at King Abdulaziz University Hospital,
            Jeddah, Saudi Arabia
        </p>
        <br>
        <h6>
            Experience
        </h6>
        <ul>
            <li>
                <h6>
                    King AbdulAziz University
                </h6>
                <p>
                    Assistant Professor, Consultant Neurosurgeon
                    <br>
                    November 2016 - Present (3 years 6 months)
                    Jeddah, Saudi Arabia
                <p>
            </li>
            <li>
                <h6>
                    Hôpital Erasme
                </h6>
                <p>
                    Skull base and vascular neurosurgery fellow
                    <br>
                    October 2017 October 2018 (1 year)
                    Brussels, Belgium
                <p>
            </li>
            <li>
                <h6>
                    APHP
                </h6>
                <p>
                    Neurosurgery resident
                    <br>
                    November 2011 November 2016 (5 years 1 month)
                    Paris, France
                <p>
            </li>
            <li>
                <h6>
                    King AbdulAziz University
                </h6>
                <p>
                    Neurosurgery resident/teaching assistant
                    <br>
                    September 2008 - September 2010 (2 years 1 month)
                    Jeddah, Saudi Arabia
                <p>
            </li>
        </ul>
    
        <h6>
            Education
        </h6>
        <ul>
            <li>
                <h6>
                    Université libre de Bruxelles
                </h6>
                <p>
                    Clinical fellowship, Skull base surgery and vascular neurosurgery
                    <br>
                    (2017 - 2018)
                    Brussels, Belgium
                <p>
            </li>
            <li>
                <h6>
                    Université René Descartes (Paris V)
                </h6>
                <p>
                    DES (Diplôme d'études spécialisées), Neurosurgery
                    <br>
                    (2011 - 2016)
                    Paris, France
                <p>
            </li>
            <li>
                <h6>
                    King Abdulaziz University
                </h6>
                <p>
                    Bachelor of Medicine, Bachelor of Surgery (MBBS), Medicine
                    <br>
                    (2001 - 2007)
                    Jeddah, Saudi Arabia
                <p>
            </li>
        </ul>
    </div>"#;
