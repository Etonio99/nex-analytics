import { useEffect, useState } from 'react';
import { ProcessSubPageProps } from '../../types/process-sub-page-props';
import { AppData } from '../../types/app-data';
import ProcessorSubPage from './processor-sub-page';
import Button from '../../components/button';
import MultiSelect, { MultiSelectItem } from '../../components/multi-select';
import { BiRightArrowAlt, BiSolidKey } from 'react-icons/bi';
// import { useNotificationContext } from '../../components/contexts/notification-context';
import Input from '../../components/input';
import Confirmation from './confirmation';

const InputRequired = (props: ProcessSubPageProps) => {
  // const { notify } = useNotificationContext();
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
        returnValue = Object.entries(selectValue)
          .filter(([_, selected]) => selected)
          .map(([key]) => Number(key));
        break;
      case 'CONFIRM':
        returnValue = true;
        break;
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

    const title = props.advanceResult.interrupt.payload?.input_field.label;
    const inputFieldType =
      props.advanceResult.interrupt.payload.input_field.data.type;

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
              label={title}
              placeholder="eCWxyomJxd56bv8.xPL7gwq..."
              icon={<BiSolidKey />}
              value={value}
              type={getInputType()}
              onChange={(e) => setValue(e.target.value)}
            />
          </>
        );
      case 'SELECT':
        const selectData =
          props.advanceResult.interrupt.payload.input_field.data.payload;

        return (
          <MultiSelect
            description={title}
            value={selectValue}
            onChange={setselectValue}
            items={selectData.options.map((item) => {
              // const addressParts = [
              //   location.street_address,
              //   location.street_address_2,
              //   location.city,
              //   location.state
              //     ? `${location.state} ${location.zip_code}`
              //     : location.zip_code,
              // ].filter(Boolean);

              // const description =
              //   addressParts.join(', ') || 'No address listed';

              return {
                label: item.title,
                description: item.subtitle,
                uniqueKey: item.key,
                displayUniqueKey: true,
              } as MultiSelectItem;
            })}
            note={
              <p className="text-xs">
                Not the locations you were expecting?{' '}
                <span
                  className="text-teal-500 cursor-pointer"
                  onClick={() => props.appActions.jumpToStep('EnterSubdomain')}
                >
                  Check your subdomain
                  <BiRightArrowAlt className="inline-block" />
                </span>
              </p>
            }
          />
        );
      case 'CONFIRM':
        return (
          <Confirmation
            confirmationData={
              props.advanceResult.interrupt.payload.input_field.data.payload
            }
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

  return (
    <ProcessorSubPage
      title={props.advanceResult.interrupt.payload.title}
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
