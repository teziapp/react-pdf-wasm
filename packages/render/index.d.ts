import React from "react";
import { PageSize, SourceObject, Style } from "../types";

declare namespace ReactPDFWasm {
    interface Styles {
        [key: string]: Style;
    }

    interface DocumentProps {
        title?: string;
    }

    
  /**
   * This component represent the PDF document itself. It must be the root
   * of your tree element structure, and under no circumstances should it be
   * used as children of another react-pdf component. In addition, it should
   * only have childs of type <Page />.
   */
  class Document extends React.Component<
    React.PropsWithChildren<DocumentProps>
  > {}
  
  interface NodeProps {
    id?: string;
    style?: Style | Style[];
  }

  interface PageProps extends NodeProps {
    size?: PageSize
  }
  /**
   * Represents single page inside the PDF document, or a subset of them if
   * using the wrapping feature. A <Document /> can contain as many pages as
   * you want, but ensure not rendering a page inside any component besides
   * Document.
   */
  class Page extends React.Component<React.PropsWithChildren<PageProps>> {}

  interface ImageWithSrcProp extends NodeProps {
    src: SourceObject;
  }

  type ImageProps = ImageWithSrcProp;

  /**
   * A React component for displaying network or local (Node only) JPG or
   * PNG images, as well as base64 encoded image strings.
   */
  class Image extends React.Component<ImageProps> {}

  interface ParagraphProps extends NodeProps {
    id?: string;
  }
  class Paragraph extends React.Component<React.PropsWithChildren<ParagraphProps>> {}

  interface LinkProps extends NodeProps {
    src: string;
  }
    /**
   * A React component for displaying a hyperlink. Link’s can be nested
   * inside a Paragraph component, or being inside any other valid primitive.
   */
    class Link extends React.Component<React.PropsWithChildren<LinkProps>> {}

    interface SpacerProps extends NodeProps {
        width: number;
        height: number
    }
    class Spacer extends React.Component<React.PropsWithChildren<SpacerProps>> {}

    // table cell can have children which may be paragraph  or any other thing
    interface TableCellProps extends NodeProps {
        style?:Styles,
        children: React.ReactNode;
    }
    class TableCell extends React.Component<React.PropsWithChildren<TableCellProps>> {} 

    interface TableRowProps extends NodeProps {
        style?:Styles,
        children: React.ReactNode;
    }
    class TableRow extends React.Component<React.PropsWithChildren<TableRowProps>> {} 
    
    interface TableProps extends NodeProps {
        style?:Styles,
        children: React.ReactNode;
    }
    class Table extends React.Component<React.PropsWithChildren<TableProps>> {} 
}