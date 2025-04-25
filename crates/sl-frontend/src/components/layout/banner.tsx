import Image from "next/image";

export default function Banner() {
  return (
    <>
      <div className="m-10">
        <Image
          src={"/banner.png"}
          className="rounded-3xl"
          alt="Banner"
          width={1600}
          height={2110}
        />
      </div>
    </>
  );
}
