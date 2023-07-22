import React, { useEffect, useRef, useState } from "react";
import IconHolder from "../elements/IconHolder";
import back from "../../assests/back.svg";
import download from "../../assests/download.svg";
import wechsel from "../../assests/WECHSEL.svg";
import dumySig from "../../assests/Jordan-signature.png";
import Pdf from "react-to-pdf";
function Bill({ identity, data, handlePage }) {
  const divRef = React.createRef();
  const [offSet, setOffSet] = useState({
    width: 0,
    height: 0,
  });
  const options = {
    orientation: "portrait",
    unit: "px",
    format: [offSet.width, offSet.height],
  };
  console.log(offSet);
  const handlePdfSize = () => {
    const divEle = document.getElementById("main-container");
    setOffSet({
      width: divEle.offsetWidth - divEle.offsetWidth / 2.29,
      height: divEle.offsetHeight - divEle.offsetWidth / 0.805,
    });
  };
  useEffect(() => {
    window.addEventListener("resize", handlePdfSize);
    return () => {
      window.removeEventListener("resize", handlePdfSize);
    };
  }, []);
  useEffect(() => {
    handlePdfSize();
  }, []);

  return (
    <div className="billing">
      <div className="top-buttons">
        <IconHolder
          handleClick={() => handlePage("repay")}
          circuled="circule"
          icon={back}
        />
        <Pdf targetRef={divRef} options={options} filename="code-example.pdf">
          {({ toPdf }) => (
            <IconHolder
              handleClick={toPdf}
              circuled="circule"
              primary="primary"
              icon={download}
            />
          )}
        </Pdf>
      </div>
      <div id="main-container" className="billing-container" ref={divRef}>
        <div className="top-container">
          <div className="head-text">
            <img src={wechsel} />
            <span>Angenommen</span>
          </div>
          <div className="line">
            <hr />
            <hr />
            <hr />
          </div>
          <div className="unter-text">
            <span>unterschrift des Annehmers</span>
          </div>
        </div>
        <div className="details">
          <div className="details-container">
            <div className="details-container-uper">
              <div className="details-container-uper-den">
                <div className="details-container-uper-den-main">
                  <div className="details-container-uper-den-main-first">
                    SOME TEXT HERE
                  </div>
                  <div className="details-container-uper-den-main-second">
                    , den
                  </div>
                  <div className="details-container-uper-den-main-third">
                    SOME TEXT HERE
                  </div>
                </div>
                <span className="bottom-text">Ort und Tag der Ausstellung</span>
              </div>
              <div className="details-container-uper-zah">
                <div className="details-container-uper-zah-main">
                  <div className="details-container-uper-zah-main-first">
                    Zahlungsort
                  </div>
                  <div className="details-container-uper-zah-main-second">
                    SOME TEXT HERE
                  </div>
                </div>
                <hr />
              </div>
            </div>
            <div className="details-container-middle">
              <div className="details-container-middle-date">
                <span className="details-container-middle-date-left">
                  Gegen diesen Wechsel - erste Ausfertigung - zahlen Sie am
                </span>
                <div className="details-container-middle-date-right">
                  <span className="details-container-middle-date-right-uper">
                    SOME NUM HERE
                  </span>
                  <span className="details-container-middle-date-right-lower">
                    Monat in Buchstaben
                  </span>
                </div>
              </div>
              <div className="details-container-middle-num">
                <span className="details-container-middle-num-text">
                  <span className="details-container-middle-num-text-an">
                    an
                  </span>
                  <span className="details-container-middle-num-text-further">
                    SOME TEXT HERE
                  </span>
                </span>
                <span className="details-container-middle-num-amount">
                  <span className="details-container-middle-num-amount-currency">
                    €
                  </span>
                  <span className="details-container-middle-num-amount-figures">
                    SOME NUM HERE
                  </span>
                </span>
              </div>
              <div className="details-container-middle-letter">
                <span className="details-container-middle-letter-currency">
                  EURO
                </span>
                <span className="details-container-middle-letter-amount">
                  <span className="details-container-middle-letter-amount-figures">
                    SOME TEXT HERE
                  </span>
                  <span className="details-container-middle-letter-amount-text">
                    Betrag in Buchstaben
                  </span>
                </span>
              </div>
            </div>
            <div className="details-container-bottom">
              <div className="details-container-bottom-left">
                <div className="details-container-bottom-left-bez">
                  <span className="details-container-bottom-left-bez-line">
                    <span className="details-container-bottom-left-bez-line-text">
                      Bezogenger
                    </span>
                    <span className="details-container-bottom-left-bez-line-ans">
                      SOME TEXT HERE
                    </span>
                  </span>
                  <span className="details-container-bottom-left-bez-next-line">
                    SOME TEXT HERE
                  </span>
                </div>
                <div className="details-container-bottom-left-in">
                  <span className="details-container-bottom-left-in-text">
                    in
                  </span>
                  <span className="details-container-bottom-left-in-further">
                    <span className="details-container-bottom-left-in-further-text">
                      SOME TEXT HERE
                    </span>
                    <span className="details-container-bottom-left-in-further-bottom">
                      Ort und Strabe (genaue Adressangebe)
                    </span>
                  </span>
                </div>
                <div className="details-container-bottom-left-detail">
                  <div className="details-container-bottom-left-bez">
                    <span className="details-container-bottom-left-bez-line">
                      <span className="details-container-bottom-left-bez-line-text">
                        Zahlbar bei
                      </span>
                      <span className="details-container-bottom-left-bez-line-ans">
                        SOME TEXT HERE
                      </span>
                    </span>
                    <span className="details-container-bottom-left-bez-next-line">
                      SOME TEXT HERE
                    </span>
                  </div>
                  <div className="details-container-bottom-left-in">
                    <span className="details-container-bottom-left-in-text">
                      in
                    </span>
                    <span className="details-container-bottom-left-in-further">
                      <span className="details-container-bottom-left-in-further-text">
                        SOME TEXT HERE
                      </span>
                      <span className="details-container-bottom-left-in-further-bottom">
                        Diesen Raum nur fur Zahistellen - und Domizilvermerke
                        benutzenl
                      </span>
                    </span>
                  </div>
                </div>
              </div>
              <div className="details-container-bottom-signature">
                <span className="signature">
                  <img src={dumySig} />
                </span>
                <span>Unterschrift und Adresse des Ausstellers</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Bill;
