use std::io;

use io::Read;
use io::Write;

use arrow_ipc::reader::StreamReader;
use arrow_ipc::writer::StreamWriter;

use arrow_array::RecordBatch;

pub fn head(b: RecordBatch, length: usize) -> Option<RecordBatch> {
    let original_len: usize = b.num_rows();
    let too_short: bool = original_len < length;
    let ok: bool = !too_short;
    ok.then_some(b.slice(0, length))
}

pub fn reader2head2writer<R, W>(rdr: R, wtr: W, length: usize) -> Result<(), io::Error>
where
    R: Read,
    W: Write,
{
    let srdr = StreamReader::try_new_buffered(rdr, None).map_err(io::Error::other)?;
    let schema = srdr.schema();
    let mut swtr = StreamWriter::try_new_buffered(wtr, &schema).map_err(io::Error::other)?;

    let mut total_rows = 0;
    for batch_result in srdr {
        let batch = batch_result.map_err(io::Error::other)?;
        let num_rows = batch.num_rows();
        if total_rows + num_rows >= length {
            let remaining = length - total_rows;
            let sliced_batch = batch.slice(0, remaining);
            swtr.write(&sliced_batch).map_err(io::Error::other)?;
            break;
        } else {
            swtr.write(&batch).map_err(io::Error::other)?;
            total_rows += num_rows;
        }
    }

    swtr.flush().map_err(io::Error::other)?;
    swtr.finish().map_err(io::Error::other)?;
    Ok(())
}

pub fn stdin2head2stdout(length: usize) -> Result<(), io::Error> {
    let rdr = io::stdin().lock();
    let mut wtr = io::stdout().lock();
    reader2head2writer(rdr, &mut wtr, length)?;
    wtr.flush()?;
    Ok(())
}
