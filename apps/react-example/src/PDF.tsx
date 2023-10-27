import { Document, Paragraph } from "@react-pdf-wasm/render"

function PDF() {
  return (
    <Document 
      title='hello Karan'
    >
      <Paragraph>
        This is Text
      </Paragraph>
    </Document>
  )
}

export default PDF