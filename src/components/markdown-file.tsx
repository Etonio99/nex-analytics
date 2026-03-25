import { useEffect, useState } from 'react';
import ReactMarkdown from 'react-markdown';
import '../css/markdown.css';

interface MarkdownFileProps {
  filePath: string;
}

const MarkdownFile = (props: MarkdownFileProps) => {
  const [content, setContent] = useState<string>('');

  useEffect(() => {
    fetch(`/content/${props.filePath}`)
      .then((res) => res.text())
      .then(setContent);
  }, [props.filePath]);

  return (
    <div className="content max-h-full h-full overflow-y-scroll rounded-md shadow-xl shadow-sandstone-950/20 p-4 bg-sandstone-50 border-4 border-sandstone-500">
      <ReactMarkdown>{content}</ReactMarkdown>
    </div>
  );
};

export default MarkdownFile;
