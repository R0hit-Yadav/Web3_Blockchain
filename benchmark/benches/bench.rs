// criterion is benchmarkinf framwork for Rust 
// prevent the compiler from optimizing away computations in a benchmark.
use criterion::{criterion_group, criterion_main, Criterion, black_box,BenchmarkGroup,Throughput};
use benchmark::{sample_data, SampleData}; // import sample data
use bincode;
use bcs;
use serde_json;
use borsh;
use rmp_serde;
use prost::Message;

fn benchmark_serialization(c: &mut Criterion) {  // for serialization
    let mut group: BenchmarkGroup<_> = c.benchmark_group("Serialization");
    let data = sample_data(); //data 


    group.throughput(Throughput::Bytes(std::mem::size_of_val(&data) as u64));

    group.bench_function("bincode serialize", |b| { // register a benchmark
        b.iter(|| bincode::serialize(black_box(&data)).unwrap()) // repeats the test multiple time
    });

    group.bench_function("bcs serialize", |b| {
        b.iter(|| bcs::to_bytes(black_box(&data)).unwrap())
    });

    group.bench_function("serde_json serialize", |b| {
        b.iter(|| serde_json::to_vec(black_box(&data)).unwrap())
    });

    group.bench_function("borsh serialize", |b| {
        b.iter(|| borsh::to_vec(black_box(&data)).unwrap())
    });

    group.bench_function("rmp serialize", |b| {
        b.iter(|| rmp_serde::to_vec(black_box(&data)).unwrap())
    });

    group.bench_function("protobuf serialize", |b| {
        b.iter(|| data.encode_to_vec()) // Protobuf serialization
    });

    group.finish();
}


fn benchmark_deserialization(c: &mut Criterion) {  // for deserialization
    let mut group: BenchmarkGroup<_> = c.benchmark_group("Deserialization");
    let data = sample_data();

    let bincode_data = bincode::serialize(&data).unwrap();
    let bcs_data = bcs::to_bytes(&data).unwrap();
    let json_data = serde_json::to_vec(&data).unwrap();
    let borsh_data = borsh::to_vec(&data).unwrap();
    let rmp_data = rmp_serde::to_vec(&data).unwrap();
    let protobuf_data = data.encode_to_vec();

    group.bench_function("bincode deserialize", |b| {
        b.iter(|| bincode::deserialize::<SampleData>(black_box(&bincode_data)).unwrap())
    });

    group.bench_function("bcs deserialize", |b| {
        b.iter(|| bcs::from_bytes::<SampleData>(black_box(&bcs_data)).unwrap())
    });

    group.bench_function("serde_json deserialize", |b| {
        b.iter(|| serde_json::from_slice::<SampleData>(black_box(&json_data)).unwrap())
    });

    group.bench_function("borsh deserialize", |b| {
        b.iter(|| borsh::from_slice::<SampleData>(black_box(&borsh_data)).unwrap())
    });

    group.bench_function("rmp deserialize", |b| {
        b.iter(|| rmp_serde::from_slice::<SampleData>(black_box(&rmp_data)).unwrap())
    });

    group.bench_function("protobuf deserialize", |b| {
        b.iter(|| SampleData::decode(black_box(&*protobuf_data)).unwrap())
    });
    group.finish();
}

fn benchmark_size(c: &mut Criterion) { // check size of data
    let data = sample_data();

    let bincode_size = bincode::serialize(&data).unwrap().len();
    let bcs_size = bcs::to_bytes(&data).unwrap().len();
    let json_size = serde_json::to_vec(&data).unwrap().len();
    let borsh_size = borsh::to_vec(&data).unwrap().len();
    let rmp_size = rmp_serde::to_vec(&data).unwrap().len();
    let protobuf_size = data.encode_to_vec().len();

    c.bench_function("size bincode", |b| b.iter(|| black_box(bincode_size)));
    c.bench_function("size bcs", |b| b.iter(|| black_box(bcs_size)));
    c.bench_function("size json", |b| b.iter(|| black_box(json_size)));
    c.bench_function("size borsh", |b| b.iter(|| black_box(borsh_size)));
    c.bench_function("size rmp", |b| b.iter(|| black_box(rmp_size)));
    c.bench_function("size protobuf", |b| b.iter(|| black_box(protobuf_size)));
}

criterion_group!(benches, benchmark_serialization, benchmark_deserialization, benchmark_size); // group all bechmarks
criterion_main!(benches);
