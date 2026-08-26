// Library function parser tests organized by category

#[path = "library/arrays/array.rs"]
mod array;

#[path = "library/arrays/filter.rs"]
mod filter;

#[path = "library/arrays/join.rs"]
mod join;

#[path = "library/arrays/lbound.rs"]
mod lbound;

#[path = "library/arrays/split.rs"]
mod split;

#[path = "library/arrays/ubound.rs"]
mod ubound;

#[path = "library/conversion/cverr.rs"]
mod cverr;

#[path = "library/conversion/hex.rs"]
mod hex;

#[path = "library/conversion/hex_dollar.rs"]
mod hex_dollar;

#[path = "library/conversion/oct.rs"]
mod oct;

#[path = "library/conversion/oct_dollar.rs"]
mod oct_dollar;

#[path = "library/conversion/vartype.rs"]
mod vartype;

#[path = "library/datetime/date.rs"]
mod date;

#[path = "library/datetime/date_dollar.rs"]
mod date_dollar;

#[path = "library/datetime/dateadd.rs"]
mod dateadd;

#[path = "library/datetime/datediff.rs"]
mod datediff;

#[path = "library/datetime/datepart.rs"]
mod datepart;

#[path = "library/datetime/dateserial.rs"]
mod dateserial;

#[path = "library/datetime/datevalue.rs"]
mod datevalue;

#[path = "library/datetime/day.rs"]
mod day;

#[path = "library/datetime/hour.rs"]
mod hour;

#[path = "library/datetime/minute.rs"]
mod minute;

#[path = "library/datetime/month.rs"]
mod month;

#[path = "library/datetime/monthname.rs"]
mod monthname;

#[path = "library/datetime/now.rs"]
mod now;

#[path = "library/datetime/second.rs"]
mod second;

#[path = "library/datetime/time.rs"]
mod time;

#[path = "library/datetime/time_dollar.rs"]
mod time_dollar;

#[path = "library/datetime/timer.rs"]
mod timer;

#[path = "library/datetime/timeserial.rs"]
mod timeserial;

#[path = "library/datetime/timevalue.rs"]
mod timevalue;

#[path = "library/datetime/weekday.rs"]
mod weekday;

#[path = "library/datetime/weekdayname.rs"]
mod weekdayname;

#[path = "library/datetime/year.rs"]
mod year;

#[path = "library/environment/environ.rs"]
mod environ;

#[path = "library/environment/environ_dollar.rs"]
mod environ_dollar;

#[path = "library/environment/error.rs"]
mod error;

#[path = "library/environment/error_dollar.rs"]
mod error_dollar;

#[path = "library/environment/getallsettings.rs"]
mod getallsettings;

#[path = "library/environment/getautoserversettings.rs"]
mod getautoserversettings;

#[path = "library/environment/getsetting.rs"]
mod getsetting;

#[path = "library/environment/imestatus.rs"]
mod imestatus;

#[path = "library/file/curdir.rs"]
mod curdir;

#[path = "library/file/curdir_dollar.rs"]
mod curdir_dollar;

#[path = "library/file/dir.rs"]
mod dir;

#[path = "library/file/eof.rs"]
mod eof;

#[path = "library/file/fileattr.rs"]
mod fileattr;

#[path = "library/file/filedatetime.rs"]
mod filedatetime;

#[path = "library/file/filelen.rs"]
mod filelen;

#[path = "library/file/freefile.rs"]
mod freefile;

#[path = "library/file/getattr.rs"]
mod getattr;

#[path = "library/file/loc.rs"]
mod loc;

#[path = "library/file/lof.rs"]
mod lof;

#[path = "library/file/seek.rs"]
mod seek;

#[path = "library/financial/ddb.rs"]
mod ddb;

#[path = "library/financial/fv.rs"]
mod fv;

#[path = "library/financial/ipmt.rs"]
mod ipmt;

#[path = "library/financial/irr.rs"]
mod irr;

#[path = "library/financial/mirr.rs"]
mod mirr;

#[path = "library/financial/nper.rs"]
mod nper;

#[path = "library/financial/npv.rs"]
mod npv;

#[path = "library/financial/pmt.rs"]
mod pmt;

#[path = "library/financial/ppmt.rs"]
mod ppmt;

#[path = "library/financial/pv.rs"]
mod pv;

#[path = "library/financial/rate.rs"]
mod rate;

#[path = "library/financial/sln.rs"]
mod sln;

#[path = "library/financial/syd.rs"]
mod syd;

#[path = "library/graphics/partition.rs"]
mod partition;

#[path = "library/graphics/qbcolor.rs"]
mod qbcolor;

#[path = "library/graphics/rgb.rs"]
mod rgb;

#[path = "library/graphics/spc.rs"]
mod spc;

#[path = "library/graphics/tab.rs"]
mod tab;

#[path = "library/interaction/command.rs"]
mod command;

#[path = "library/interaction/command_dollar.rs"]
mod command_dollar;

#[path = "library/interaction/doevents.rs"]
mod doevents;

#[path = "library/interaction/input.rs"]
mod input;

#[path = "library/interaction/inputbox.rs"]
mod inputbox;

#[path = "library/interaction/msgbox.rs"]
mod msgbox;

#[path = "library/interaction/shell.rs"]
mod shell;

#[path = "library/logic/choose.rs"]
mod choose;

#[path = "library/logic/iif.rs"]
mod iif;

#[path = "library/logic/switch.rs"]
mod switch;

#[path = "library/math/abs.rs"]
mod abs;

#[path = "library/math/atn.rs"]
mod atn;

#[path = "library/math/cos.rs"]
mod cos;

#[path = "library/math/exp.rs"]
mod exp;

#[path = "library/math/fix.rs"]
mod fix;

#[path = "library/math/int.rs"]
mod int;

#[path = "library/math/log.rs"]
mod log;

#[path = "library/math/rnd.rs"]
mod rnd;

#[path = "library/math/round.rs"]
mod round;

#[path = "library/math/sgn.rs"]
mod sgn;

#[path = "library/math/sin.rs"]
mod sin;

#[path = "library/math/sqr.rs"]
mod sqr;

#[path = "library/math/tan.rs"]
mod tan;

#[path = "library/objects/callbyname.rs"]
mod callbyname;

#[path = "library/objects/createobject.rs"]
mod createobject;

#[path = "library/objects/getobject.rs"]
mod getobject;

#[path = "library/objects/typename.rs"]
mod typename;

#[path = "library/resources/loadpicture.rs"]
mod loadpicture;

#[path = "library/resources/loadresdata.rs"]
mod loadresdata;

#[path = "library/resources/loadrespicture.rs"]
mod loadrespicture;

#[path = "library/resources/loadresstring.rs"]
mod loadresstring;

#[path = "library/string/asc.rs"]
mod asc;

#[path = "library/string/ascb.rs"]
mod ascb;

#[path = "library/string/ascw.rs"]
mod ascw;

#[path = "library/string/chr.rs"]
mod chr;

#[path = "library/string/chr_dollar.rs"]
mod chr_dollar;

#[path = "library/string/chrb.rs"]
mod chrb;

#[path = "library/string/chrb_dollar.rs"]
mod chrb_dollar;

#[path = "library/string/chrw.rs"]
mod chrw;

#[path = "library/string/chrw_dollar.rs"]
mod chrw_dollar;

#[path = "library/string/format.rs"]
mod format;

#[path = "library/string/format_dollar.rs"]
mod format_dollar;

#[path = "library/string/formatcurrency.rs"]
mod formatcurrency;

#[path = "library/string/formatdatetime.rs"]
mod formatdatetime;

#[path = "library/string/formatnumber.rs"]
mod formatnumber;

#[path = "library/string/formatpercent.rs"]
mod formatpercent;

#[path = "library/string/instr.rs"]
mod instr;

#[path = "library/string/instrrev.rs"]
mod instrrev;

#[path = "library/string/lcase.rs"]
mod lcase;

#[path = "library/string/lcase_dollar.rs"]
mod lcase_dollar;

#[path = "library/string/left.rs"]
mod left;

#[path = "library/string/left_dollar.rs"]
mod left_dollar;

#[path = "library/string/leftb.rs"]
mod leftb;

#[path = "library/string/leftb_dollar.rs"]
mod leftb_dollar;

#[path = "library/string/len.rs"]
mod len;

#[path = "library/string/lenb.rs"]
mod lenb;

#[path = "library/string/ltrim.rs"]
mod ltrim;

#[path = "library/string/ltrim_dollar.rs"]
mod ltrim_dollar;

#[path = "library/string/mid.rs"]
mod mid;

#[path = "library/string/mid_dollar.rs"]
mod mid_dollar;

#[path = "library/string/midb.rs"]
mod midb;

#[path = "library/string/midb_dollar.rs"]
mod midb_dollar;

#[path = "library/string/replace.rs"]
mod replace;

#[path = "library/string/right.rs"]
mod right;

#[path = "library/string/right_dollar.rs"]
mod right_dollar;

#[path = "library/string/rightb.rs"]
mod rightb;

#[path = "library/string/rightb_dollar.rs"]
mod rightb_dollar;

#[path = "library/string/rtrim.rs"]
mod rtrim;

#[path = "library/string/rtrim_dollar.rs"]
mod rtrim_dollar;

#[path = "library/string/space.rs"]
mod space;

#[path = "library/string/space_dollar.rs"]
mod space_dollar;

#[path = "library/string/str.rs"]
mod str;

#[path = "library/string/str_dollar.rs"]
mod str_dollar;

#[path = "library/string/strcomp.rs"]
mod strcomp;

#[path = "library/string/strconv.rs"]
mod strconv;

#[path = "library/string/string_function.rs"]
mod string_function;

#[path = "library/string/strreverse.rs"]
mod strreverse;

#[path = "library/string/trim.rs"]
mod trim;

#[path = "library/string/trim_dollar.rs"]
mod trim_dollar;

#[path = "library/string/ucase.rs"]
mod ucase;

#[path = "library/string/ucase_dollar.rs"]
mod ucase_dollar;

#[path = "library/type_checking/isarray.rs"]
mod isarray;

#[path = "library/type_checking/isdate.rs"]
mod isdate;

#[path = "library/type_checking/isempty.rs"]
mod isempty;

#[path = "library/type_checking/iserror.rs"]
mod iserror;

#[path = "library/type_checking/ismissing.rs"]
mod ismissing;

#[path = "library/type_checking/isnull.rs"]
mod isnull;

#[path = "library/type_checking/isnumeric.rs"]
mod isnumeric;

#[path = "library/type_checking/isobject.rs"]
mod isobject;
