#pragma once
#include <vector>
#include <cstdint>
#include <cstddef>

namespace org::blobstore {
    struct MultiBuf {
        std::vector<std::vector<uint8_t>> chunks;
        size_t pos;

        bool has_more_chunks() const;
        std::vector<uint8_t>& next_chunk();
    };
}
