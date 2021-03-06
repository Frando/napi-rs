use super::napi_status;

#[derive(Eq, PartialEq, Debug)]
pub enum Status {
  Ok,
  InvalidArg,
  ObjectExpected,
  StringExpected,
  NameExpected,
  FunctionExpected,
  NumberExpected,
  BooleanExpected,
  ArrayExpected,
  GenericFailure,
  PendingException,
  Cancelled,
  EscapeCalledTwice,
  HandleScopeMismatch,
  CallbackScopeMismatch,
  StringContainsNull,
  QueueFull,
  Closing,
  BigintExpected,
  Unknown,
}

impl From<napi_status> for Status {
  fn from(code: napi_status) -> Self {
    use self::napi_status::*;
    use Status::*;

    match code {
      napi_ok => Ok,
      napi_invalid_arg => InvalidArg,
      napi_object_expected => ObjectExpected,
      napi_string_expected => StringExpected,
      napi_name_expected => NameExpected,
      napi_function_expected => FunctionExpected,
      napi_number_expected => NumberExpected,
      napi_boolean_expected => BooleanExpected,
      napi_array_expected => ArrayExpected,
      napi_generic_failure => GenericFailure,
      napi_pending_exception => PendingException,
      napi_cancelled => Cancelled,
      napi_escape_called_twice => EscapeCalledTwice,
      napi_handle_scope_mismatch => HandleScopeMismatch,
      napi_callback_scope_mismatch => CallbackScopeMismatch,
      napi_queue_full => QueueFull,
      napi_closing => Closing,
      napi_bigint_expected => BigintExpected,
      _ => Unknown,
    }
  }
}
