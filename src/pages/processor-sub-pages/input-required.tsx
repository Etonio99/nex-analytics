import { useEffect, useState } from 'react';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { AppData } from '../../types/app-data';
import ProcessorSubPage from './processor-sub-page';
import Button from '../../components/button';
import MultiSelect, { MultiSelectItem } from '../../components/multi-select';
import { BiRightArrowAlt, BiSolidKey } from 'react-icons/bi';
import { useNotificationContext } from '../../components/contexts/notification-context';
import Input from '../../components/input';
import Confirmation from '../../components/confirmation';

const InputRequired = (props: ProcessSubPageProps) => {
  const { notify } = useNotificationContext();
  const [value, setValue] = useState<string | undefined>();
  const [selectValue, setselectValue] = useState<Record<string, boolean>>({});

  if (
    props.advanceResult?.interrupt?.type !== 'INPUT_REQUIRED' ||
    !props.advanceResult?.interrupt?.payload
  ) {
    return null;
  }

  const continueProcess = async () => {
    if (
      props.advanceResult?.interrupt?.type !== 'INPUT_REQUIRED' ||
      !props.advanceResult?.interrupt?.payload
    ) {
      return null;
    }

    const key = props.advanceResult.interrupt.payload.input_field.key;
    let returnValue = undefined;
    switch (props.advanceResult.interrupt.payload.input_field.data.type) {
      case 'STRING':
      case 'DATE':
        returnValue = value;
        break;
      case 'NUMBER':
        returnValue = value !== undefined ? Number(value) : undefined;
        break;
      case 'SELECT':
        const selections = Object.entries(selectValue)
          .filter(([_, selected]) => selected)
          .map(([key]) => Number(key));
        returnValue = selections.length > 0 ? selections : undefined;
        break;
      case 'CONFIRM':
        returnValue = true;
        break;
    }
    if (
      props.advanceResult.interrupt.payload.input_field.required &&
      !returnValue
    ) {
      notify(
        'Input Required',
        `"${props.advanceResult.interrupt.payload.input_field.label}" is required`
      );
      return;
    }

    const appDataKeys = ['subdomain'];
    if (appDataKeys.includes(key)) {
      await props.appActions.updateAppData({ [key]: returnValue } as AppData);
    } else {
      await props.appActions.updateProcessorData({ [key]: returnValue });
    }
    await props.appActions.advanceProcessor();
  };

  const getInputField = () => {
    if (
      props.advanceResult?.interrupt?.type !== 'INPUT_REQUIRED' ||
      !props.advanceResult?.interrupt?.payload
    ) {
      return null;
    }

    const inputField = props.advanceResult.interrupt.payload?.input_field;
    const inputFieldType = inputField.data.type;

    switch (inputFieldType) {
      case 'STRING':
      case 'DATE':
      case 'NUMBER':
        const getInputType = () => {
          switch (inputFieldType) {
            case 'DATE':
              return 'date';
            case 'NUMBER':
              return 'number';
            default:
              return 'text';
          }
        };

        return (
          <>
            <Input
              label={inputField.label}
              placeholder={inputField.placeholder ?? ''}
              icon={<BiSolidKey />}
              value={value}
              type={getInputType()}
              onChange={(e) => setValue(e.target.value)}
            />
          </>
        );
      case 'SELECT':
        const selectData = inputField.data.payload;

        const getNote = () => {
          switch (inputField.key) {
            case 'subdomain':
              return (
                <p className="text-xs">
                  Not the locations you were expecting?{' '}
                  <span
                    className="text-teal-500 cursor-pointer"
                    onClick={() =>
                      props.appActions.jumpToStep('EnterSubdomain')
                    }
                  >
                    Check your subdomain
                    <BiRightArrowAlt className="inline-block" />
                  </span>
                </p>
              );
            default:
              return null;
          }
        };

        return (
          <MultiSelect
            title={inputField.label}
            description={inputField.description}
            value={selectValue}
            onChange={setselectValue}
            items={selectData.options.map((item) => {
              return {
                label: item.title,
                description: item.subtitle,
                uniqueKey: item.key,
                displayUniqueKey: true,
              } as MultiSelectItem;
            })}
            note={getNote()}
          />
        );
      case 'CONFIRM':
        return (
          <Confirmation
            confirmationData={inputField.data.payload}
            jumpToStep={props.appActions.jumpToStep}
          />
        );
    }
  };

  useEffect(() => {
    if (
      props.advanceResult?.interrupt?.type === 'INPUT_REQUIRED' &&
      props.advanceResult?.interrupt?.payload
    )
      switch (props.advanceResult?.interrupt?.payload?.input_field.data.type) {
        case 'STRING':
        case 'NUMBER':
          const initialValue =
            props.advanceResult.interrupt.payload.input_field.data.payload ??
            '';
          setValue(initialValue.toString());
          break;
        case 'DATE':
          const initialDate =
            props.advanceResult.interrupt.payload.input_field.data.payload ??
            new Date().toISOString().split('T')[0];
          setValue(initialDate);
          break;
        case 'SELECT':
          const options: Record<string, boolean> = {};
          const selectedOptionKeys =
            props.advanceResult.interrupt.payload.input_field.data.payload
              .selected_keys ?? [];
          props.advanceResult.interrupt.payload.input_field.data.payload.options.forEach(
            (option) => {
              options[option.key] = selectedOptionKeys.includes(option.key);
            }
          );
          setselectValue(options);
          break;
      }
  }, []);

  const getTitleIcon = () => {
    if (
      props.advanceResult?.interrupt?.type !== 'INPUT_REQUIRED' ||
      !props.advanceResult?.interrupt?.payload
    ) {
      return null;
    }

    switch (props.advanceResult.interrupt.payload.input_field.data.type) {
      case 'CONFIRM':
        return <img src="nexie/nexie-search.png" className="pixelated h-10" />;
      default:
        return null;
    }
  };

  return (
    <ProcessorSubPage
      title={props.advanceResult.interrupt.payload.title}
      titleIcon={getTitleIcon()}
      description={props.advanceResult.interrupt.payload.description}
      appActions={props.appActions}
    >
      {getInputField()}
      <div className="mt-2 flex justify-end items-center gap-2">
        <p className="text-sandstone-300 italic">
          {props.advanceResult.interrupt.payload?.input_field.description}
        </p>
        <Button label="Save" style="primary" onClick={continueProcess} />
      </div>
    </ProcessorSubPage>
  );
};

export default InputRequired;
